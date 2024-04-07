use std::collections::BTreeMap;

use crate::normalize_context::ast;
use crate::shared::ast::Program;
use crate::simplify_values::ast as input;

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

    fn exp(&mut self, e: input::Exp) -> ast::Exp {
        match e {
            input::Exp::Triv(t) => ast::Exp::Triv(t),
            input::Exp::Call(target, args) => ast::Exp::Call(target, args),
            input::Exp::If(test, conseq, alt) => {
                ast::Exp::If(test, self.bexp(conseq), self.bexp(alt))
            }
            input::Exp::Seq(stmts, body) => {
                let block = self.stmt_block(stmts);
                make_block(block, *self.bexp(body))
            }
        }
    }

    fn bexp(&mut self, e: Box<input::Exp>) -> Box<ast::Exp> {
        Box::new(self.exp(*e))
    }

    fn stmt_exp(&mut self, block: &mut Vec<ast::Stmt>, e: input::Exp) {
        match e {
            input::Exp::Call(subject, args) => {
                block.push(ast::Stmt::Call(subject, args));
            },
            input::Exp::Seq(stmts, e) => {
                for stmt in stmts {
                    self.stmt(block, stmt);
                }
                self.stmt_exp(block, *e);
            },
            input::Exp::If(test, conseq, alt) => {
                let mut conseq_block = vec![];
                self.stmt_exp(&mut  conseq_block, *conseq);
                let mut alt_block = vec![];
                self.stmt_exp(&mut  alt_block, *alt);
                block.push(ast::Stmt::If(test, conseq_block, alt_block));
            },
            input::Exp::Triv(_) => (),
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
            input::Stmt::Exp(e) => self.stmt_exp(block, *e),
            input::Stmt::Let(x, rhs) => match *rhs {
                input::Exp::Call(subject, args) => {
                    block.push(ast::Stmt::LetCall(x, subject, args));
                }
                input::Exp::Seq(stmts, last) => {
                    for stmt in stmts {
                        self.stmt(block, stmt);
                    }
                    self.stmt(block, input::Stmt::Let(x, last));
                },
                input::Exp::If(test, conseq, alt) => {
                    let mut conseq_block = vec![];
                    self.stmt(&mut conseq_block, input::Stmt::Let(x.clone(), conseq));
                    let mut alt_block = vec![];
                    self.stmt(&mut alt_block, input::Stmt::Let(x, alt));
                    block.push(ast::Stmt::If(test, conseq_block, alt_block));
                },
                input::Exp::Triv(t) => {
                    block.push(ast::Stmt::Let(x, t));
                }
            }
            input::Stmt::LetBinop(x, lhs, op, rhs) => {
                block.push(ast::Stmt::LetBinop(x, lhs, op, rhs));
            }
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
