use std::collections::BTreeMap;

use crate::input;
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
            output_funcs.insert(name, pass.exp(body));
        }
        Program {
            funcs: output_funcs,
        }
    }

    fn make_tmp(&mut self) -> Var {
        self.counter += 1;
        format!("tmp.{}", self.counter)
    }

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
                block.push(ast::Stmt::LetBinop(tmp.clone(), new_lhs, op, new_rhs));
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

    // fully for effect; pushes statements onto the block
    fn stmt(&mut self, block: &mut Vec<ast::Stmt>, s: input::Stmt) {
        match s {
            input::Stmt::Exp(e) => block.push(ast::Stmt::Exp(Box::new(self.exp(*e)))),
            input::Stmt::Let(x, rhs) => block.push(ast::Stmt::Let(x, Box::new(self.exp(*rhs)))),
        }
    }

    fn triv(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) -> Triv {
        match e {
            hoist @ (input::Exp::Call(_, _)
            | input::Exp::Seq(_, _)
            | input::Exp::Binop(_, _, _)
            | input::Exp::If(_, _, _)) => {
                let tmp = self.make_tmp();
                block.push(ast::Stmt::Let(tmp.clone(), Box::new(self.exp(hoist))));
                Triv::Var(tmp)
            }
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
            ast::Exp::Seq(stmts, base)
        }
        exp @ (ast::Exp::Call(_, _) | ast::Exp::If(_, _, _) | ast::Exp::Triv(_)) => {
            ast::Exp::Seq(block, Box::new(exp))
        }
    }
}
