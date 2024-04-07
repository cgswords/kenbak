use std::collections::BTreeMap;

use crate::introduce_call_conventions::ast;
use crate::shared::ast::{Program, Triv};
use crate::normalize_context::ast as input;

pub struct Pass {
    counter: u32,
}

impl Pass {
    pub fn run(program: Program<input::Exp>) -> Program<ast::Exp> {
        let Program { funcs } = program;
        let mut pass = Pass { counter: 0 };
        let mut output_funcs = BTreeMap::new();
        for (name, body) in funcs {
            output_funcs.insert(name, pass.tail(body));
        }
        Program {
            funcs: output_funcs,
        }
    }

    fn tail(&mut self, e: input::Exp) -> ast::Exp {
        match e {
            input::Exp::Call(target, args) => {
                // TCO baby!
                let mut block = vec![];
                push_args(&mut block, args);
                make_block(block, ast::Exp::Call(target))
            }
            input::Exp::Seq(stmts, e) => {
                let block = self.stmt_block(stmts);
                make_block(block, self.tail(*e))
            }
            input::Exp::If(test, conseq, alt) => {
                ast::Exp::If(test, self.btail(conseq), self.btail(alt))
            }
            input::Exp::Triv(t) => {
                let block = vec![ast::Stmt::ReturnSet(t)];
                make_block(block, ast::Exp::Return)
            }
        }
    }

    fn btail(&mut self, e: Box<input::Exp>) -> Box<ast::Exp> {
        Box::new(self.tail(*e))
    }

    fn value(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) -> Triv {
        match e {
            input::Exp::Call(target, args) => {
                // We put them on the stack backwards for our sanity.
                for arg in args.into_iter().rev() {
                    block.push(ast::Stmt::Push(arg));
                }
                block.push(ast::Stmt::Exp(Box::new(ast::Exp::Call(target))));
                Triv::Return
            }
            input::Exp::Seq(stmts, e) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
                self.value(block, *e)
            }
            input::Exp::If(_test, _conseq, _alt) => todo!(),
            input::Exp::Triv(t) => t,
        }
    }

    fn  stmt_block(&mut self, block: Vec<input::Stmt>) -> Vec<ast::Stmt> {
        let mut output_block = vec![];
        for stmt  in block {
            self.stmt(&mut output_block, stmt);
        }
        output_block
    }

    // fully for effect; pushes statements onto the block
    fn stmt(&mut self, block: &mut Vec<ast::Stmt>, s: input::Stmt) {
        match s {
            input::Stmt::Let(x, rhs) => {
                block.push(ast::Stmt::Let(x, rhs));
            }
            input::Stmt::LetBinop(x, lhs, op, rhs) => {
                block.push(ast::Stmt::LetBinop(x, lhs, op, rhs))
            }
            input::Stmt::LetCall(x, subject, args) => {
                push_args(block, args);
                block.push(ast::Stmt::Call(subject));
                block.push(ast::Stmt::Let(x, Triv::Return));
            },
            input::Stmt::If(test, conseq, alt) => {
                block.push(ast::Stmt::If(test, self.stmt_block(conseq), self.stmt_block(alt)));
            },
            input::Stmt::Call(subject, args) => {
                push_args(block, args);
                block.push(ast::Stmt::Call(subject))
            },
        }
    }
}

fn push_args(block: &mut Vec<ast::Stmt>, args: Vec<Triv>) {
    // We put them on the stack backwards for our sanity.
    for arg in args.into_iter().rev() {
        block.push(ast::Stmt::Push(arg));
    }
}

fn make_block(block: Vec<ast::Stmt>, exp: ast::Exp) -> ast::Exp {
    match exp {
        exp if block.is_empty() => exp,
        ast::Exp::Seq(stmts, base) => {
            let stmts = block.into_iter().chain(stmts.into_iter()).collect();
            ast::Exp::Seq(stmts, base)
        }
        exp @ (ast::Exp::Call(_) | ast::Exp::If(_, _, _) | ast::Exp::Return) => {
            ast::Exp::Seq(block, Box::new(exp))
        }
    }
}
