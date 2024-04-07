use std::collections::BTreeMap;

use crate::normalize_context::ast as input;
use crate::shared::ast::{Program, Triv, Var};
use crate::simplify_values::ast;

pub struct Pass {
    counter: u32,
}

impl Pass {
    pub fn run(program: Program<input::Exp>) -> Program<ast::Exp> {
        let Program { funcs } = program;
        let mut pass = Pass { counter: 0 };
        let mut output_funcs = BTreeMap::new();
        for (name, body) in funcs {
            output_funcs.insert(name, *pass.exp_block(Box::new(body)));
        }
        Program {
            funcs: output_funcs,
        }
    }

    fn make_tmp(&mut self) -> Var {
        self.counter += 1;
        format!("tmp.{}", self.counter)
    }

    fn exp(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) -> ast::Exp {
        match e {
            input::Exp::Call(subject, args) => {
                let new_subject = self.triv(block, *subject);
                let new_args = args
                    .into_iter()
                    .map(|arg| self.triv(block, arg))
                    .collect::<Vec<_>>();
                ast::Exp::Call(new_subject, new_args)
            }
            input::Exp::Seq(stmts, body) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
                self.exp(block, *body)
            }
            input::Exp::Binop(lhs, op, rhs) => {
                let lhs = match self.triv(block, *lhs) {
                    Triv::Var(x) => x,
                    v @ Triv::Value(_) => {
                        let new_tmp = self.make_tmp();
                        block.push(ast::Stmt::Let(new_tmp.clone(), Box::new(ast::Exp::Triv(v))));
                        new_tmp
                    }
                    Triv::Return => unreachable!(),
                };
                let rhs = self.triv(block, *rhs);
                ast::Exp::Binop(lhs, op, rhs)
            }
            input::Exp::If(test, conseq, alt) => ast::Exp::If(
                self.bpred(block, test),
                self.exp_block(conseq),
                self.exp_block(alt),
            ),
            input::Exp::Value(v) => ast::Exp::Triv(Triv::Value(v)),
            input::Exp::Var(x) => ast::Exp::Triv(Triv::Var(x)),
        }
    }

    fn exp_block(&mut self, e: Box<input::Exp>) -> Box<ast::Exp> {
        let mut block = vec![];
        let e = self.exp(&mut block, *e);
        Box::new(make_block(block, e))
    }

    fn bexp(&mut self, block: &mut Vec<ast::Stmt>, e: Box<input::Exp>) -> Box<ast::Exp> {
        Box::new(self.exp(block, *e))
    }

    fn pred(&mut self, block: &mut Vec<ast::Stmt>, p: input::Pred) -> ast::Pred {
        match p {
            input::Pred::Call(subject, args) => {
                let new_subject = self.triv(block, *subject);
                let new_args = args
                    .into_iter()
                    .map(|arg| self.triv(block, arg))
                    .collect::<Vec<_>>();
                ast::Pred::Call(new_subject, new_args)
            }
            input::Pred::Seq(stmts, body) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
                self.pred(block, *body)
            }
            input::Pred::Relop(lhs, op, rhs) => {
                let lhs = match self.triv(block, *lhs) {
                    Triv::Var(x) => x,
                    v @ Triv::Value(_) => {
                        let new_tmp = self.make_tmp();
                        block.push(ast::Stmt::Let(new_tmp.clone(), Box::new(ast::Exp::Triv(v))));
                        new_tmp
                    }
                    Triv::Return => unreachable!(),
                };
                let rhs = self.triv(block, *rhs);
                ast::Pred::Relop(lhs, op, rhs)
            }
            input::Pred::If(test, conseq, alt) => ast::Pred::If(
                self.bpred(block, test),
                self.pred_block(conseq),
                self.pred_block(alt),
            ),
            input::Pred::True => ast::Pred::True,
            input::Pred::False => ast::Pred::False,
        }
    }

    fn pred_block(&mut self, e: Box<input::Pred>) -> Box<ast::Pred> {
        let mut block = vec![];
        let e = self.pred(&mut block, *e);
        Box::new(make_pred_block(block, e))
    }

    fn bpred(&mut self, block: &mut Vec<ast::Stmt>, p: Box<input::Pred>) -> Box<ast::Pred> {
        Box::new(self.pred(block, *p))
    }
    /*
    fn exp(&mut self, e: input::Exp) -> ast::Exp {
        match e {
            input::Exp::Call(target, args) => {
                let mut block = vec![];
                let new_target = self.triv(&mut block, *target);
                let new_args = args
                    .into_iter()
                    .map(|arg| self.triv(&mut block, arg))
                    .collect::<Vec<_>>();
                make_block(block, ast::Exp::Call(new_target, new_args))
            }
            input::Exp::Seq(stmts, e) => {
                let mut block = vec![];
                for stmt in stmts {
                    self.stmt(&mut block, stmt);
                }
                make_block(block, self.exp(*e))
            }
            input::Exp::Binop(lhs, op, rhs) => {
                let mut block = vec![];
                let new_lhs = self.triv(&mut block, *lhs);
                let new_rhs = self.triv(&mut block, *rhs);
                let tmp = self.make_tmp();
                block.push(ast::Stmt::Let(tmp.clone(), Box::new(ast::Exp::Triv(new_lhs))));
                block.push(ast::Stmt::LetBinop(tmp.clone(), op, new_rhs));
                make_block(block, ast::Exp::Triv(Triv::Var(tmp)))
            }
            input::Exp::If(test, conseq, alt) => {
                let mut block = vec![];
                let new_test = self.triv(&mut block, *test);
                make_block(
                    block,
                    ast::Exp::If(
                        new_test,
                        Box::new(self.exp(*conseq)),
                        Box::new(self.exp(*alt)),
                    ),
                )
            }
            input::Exp::Value(v) => ast::Exp::Triv(Triv::Value(v)),
            input::Exp::Var(x) => ast::Exp::Triv(Triv::Var(x)),
        }
    }
    */

    // fully for effect; pushes statements onto the block
    fn stmt(&mut self, block: &mut Vec<ast::Stmt>, s: input::Stmt) {
        match s {
            input::Stmt::Exp(e) => {
                let e = self.bexp(block, e);
                block.push(ast::Stmt::Exp(e));
            }
            input::Stmt::Let(x, rhs) => {
                let rhs = self.bexp(block, rhs);
                block.push(ast::Stmt::Let(x, rhs));
            }
            input::Stmt::If(test, conseq, alt) => {
                let test = self.bpred(block, test);
                block.push(ast::Stmt::If(
                    test,
                    self.stmt_block(conseq),
                    self.stmt_block(alt),
                ));
            }
            input::Stmt::Seq(stmts) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
            }
        }
    }

    fn stmt_block(&mut self, stmts: Vec<input::Stmt>) -> Vec<ast::Stmt> {
        let mut block = vec![];
        for stmt in stmts {
            self.stmt(&mut block, stmt);
        }
        block
    }

    fn triv(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) -> Triv {
        match e {
            hoist @ (input::Exp::Call(_, _)
            | input::Exp::Seq(_, _)
            | input::Exp::Binop(_, _, _)
            | input::Exp::If(_, _, _)) => match self.exp(block, hoist) {
                ast::Exp::Triv(t) => t,
                exp @ (ast::Exp::Call(_, _)
                | ast::Exp::Seq(_, _)
                | ast::Exp::If(_, _, _)
                | ast::Exp::Binop(_, _, _)) => {
                    let tmp = self.make_tmp();
                    block.push(ast::Stmt::Let(tmp.clone(), Box::new(exp)));
                    Triv::Var(tmp)
                }
            },
            input::Exp::Value(v) => Triv::Value(v),
            input::Exp::Var(x) => Triv::Var(x),
        }
    }
}

fn make_block(block: Vec<ast::Stmt>, exp: ast::Exp) -> ast::Exp {
    match exp {
        exp if block.is_empty() => exp,
        ast::Exp::Seq(stmts, base) => {
            let stmts = block.into_iter().chain(stmts.into_iter()).collect();
            make_block(stmts, *base)
        }
        exp @ (ast::Exp::Call(_, _)
        | ast::Exp::If(_, _, _)
        | ast::Exp::Triv(_)
        | ast::Exp::Binop(_, _, _)) => ast::Exp::Seq(block, Box::new(exp)),
    }
}

fn make_pred_block(block: Vec<ast::Stmt>, exp: ast::Pred) -> ast::Pred {
    match exp {
        exp if block.is_empty() => exp,
        ast::Pred::Seq(stmts, base) => {
            let stmts = block.into_iter().chain(stmts.into_iter()).collect();
            make_pred_block(stmts, *base)
        }
        exp @ (ast::Pred::Call(_, _)
        | ast::Pred::If(_, _, _)
        | ast::Pred::Triv(_)
        | ast::Pred::False
        | ast::Pred::True
        | ast::Pred::Relop(_, _, _)) => ast::Pred::Seq(block, Box::new(exp)),
    }
}
