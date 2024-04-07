use std::collections::BTreeMap;

use crate::input;
use crate::normalize_context::ast;
use crate::shared::ast::{Op, Program, Value};

pub struct Pass {
    counter: u32,
}

impl Pass {
    pub fn run(program: Program<input::Exp>) -> Program<ast::Exp> {
        let Program { funcs } = program;
        let mut pass = Pass { counter: 0 };
        let mut output_funcs = BTreeMap::new();
        for (name, body) in funcs {
            output_funcs.insert(name, pass.value(body));
        }
        Program {
            funcs: output_funcs,
        }
    }

    fn value(&mut self, e: input::Exp) -> ast::Exp {
        match e {
            input::Exp::Call(subject, args) => ast::Exp::Call(
                self.bvalue(subject),
                args.into_iter().map(|arg| self.value(arg)).collect(),
            ),
            input::Exp::Seq(stmts, value) => {
                let stmts = self.stmts(stmts);
                make_block(stmts, self.value(*value))
            }
            input::Exp::Binop(lhs, op, rhs) => match op {
                Op::Add | Op::Sub => ast::Exp::Binop(self.bvalue(lhs), op, self.bvalue(rhs)),
                Op::Eq | Op::Neq => ast::Exp::If(
                    Box::new(ast::Pred::Relop(self.bvalue(lhs), op, self.bvalue(rhs))),
                    Box::new(ast::Exp::Value(Value::True)),
                    Box::new(ast::Exp::Value(Value::False)),
                ),
            },
            input::Exp::If(test, conseq, alt) => {
                ast::Exp::If(self.bpred(test), self.bvalue(conseq), self.bvalue(alt))
            }
            input::Exp::Value(v) => ast::Exp::Value(v),
            input::Exp::Var(x) => ast::Exp::Var(x),
        }
    }

    fn bvalue(&mut self, e: Box<input::Exp>) -> Box<ast::Exp> {
        Box::new(self.value(*e))
    }

    fn pred(&mut self, e: input::Exp) -> ast::Pred {
        match e {
            // We didn't type-check, so scheme truthiness for now
            e @ (input::Exp::Call(_, _) | input::Exp::Value(_) | input::Exp::Var(_)) => {
                ast::Pred::If(
                    Box::new(ast::Pred::Relop(
                        Box::new(self.value(e)),
                        Op::Eq,
                        Box::new(ast::Exp::Value(Value::False)),
                    )),
                    Box::new(ast::Pred::False),
                    Box::new(ast::Pred::True),
                )
            }
            input::Exp::Seq(stmts, value) => {
                let stmts = self.stmts(stmts);
                ast::Pred::Seq(stmts, self.bpred(value))
            }
            input::Exp::Binop(lhs, op, rhs) => match &op {
                Op::Add | Op::Sub => ast::Pred::If(
                    Box::new(ast::Pred::Relop(
                        Box::new(self.value(input::Exp::Binop(lhs, op, rhs))),
                        Op::Eq,
                        Box::new(ast::Exp::Value(Value::False)),
                    )),
                    Box::new(ast::Pred::False),
                    Box::new(ast::Pred::True),
                ),
                Op::Eq | Op::Neq => ast::Pred::Relop(self.bvalue(lhs), op, self.bvalue(rhs)),
            },
            input::Exp::If(test, conseq, alt) => {
                ast::Pred::If(self.bpred(test), self.bpred(conseq), self.bpred(alt))
            }
        }
    }

    fn bpred(&mut self, e: Box<input::Exp>) -> Box<ast::Pred> {
        Box::new(self.pred(*e))
    }

    fn stmts(&mut self, ss: Vec<input::Stmt>) -> Vec<ast::Stmt> {
        let mut block = vec![];
        for s in ss {
            self.stmt(&mut block, s);
        }
        block
    }

    fn stmt(&mut self, block: &mut Vec<ast::Stmt>, s: input::Stmt) {
        match s {
            input::Stmt::Exp(e) => self.stmt_expr(block, *e),
            input::Stmt::Let(x, e) => block.push(ast::Stmt::Let(x, self.bvalue(e))),
        }
    }

    fn stmt_expr(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) {
        match e {
            e @ input::Exp::Call(_, _) => {
                block.push(ast::Stmt::Exp(Box::new(self.value(e))));
            }
            input::Exp::Seq(stmts, end) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
                self.stmt_expr(block, *end);
            }
            input::Exp::Binop(lhs, _, rhs) => {
                self.stmt_expr(block, *lhs);
                self.stmt_expr(block, *rhs);
            }
            input::Exp::If(test, conseq, alt) => {
                let mut conseq_block = vec![];
                self.stmt_expr(&mut conseq_block, *conseq);
                let mut alt_block = vec![];
                self.stmt_expr(&mut alt_block, *alt);
                block.push(ast::Stmt::If(self.bpred(test), conseq_block, alt_block));
            }
            input::Exp::Value(_) | input::Exp::Var(_) => (),
        }
    }
}

fn make_block(block: Vec<ast::Stmt>, exp: ast::Exp) -> ast::Exp {
    match exp {
        exp if block.is_empty() => exp,
        ast::Exp::Seq(stmts, base) => {
            let stmts = block.into_iter().chain(stmts.into_iter()).collect();
            ast::Exp::Seq(stmts, base)
        }
        exp @ (ast::Exp::Call(_, _)
        | ast::Exp::If(_, _, _)
        | ast::Exp::Var(_)
        | ast::Exp::Value(_)
        | ast::Exp::Binop(_, _, _)) => ast::Exp::Seq(block, Box::new(exp)),
    }
}
