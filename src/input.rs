use std::collections::BTreeMap;
use std::fmt;

use crate::shared::ast::{Op, Value, Var};

#[derive(Clone)]
pub enum Exp {
    Call(Box<Exp>, Vec<Exp>),
    Seq(Vec<Stmt>, Box<Exp>),
    Binop(Box<Exp>, Op, Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
    Value(Value),
    Var(Var),
}

#[derive(Clone)]
pub enum Stmt {
    Exp(Box<Exp>),
    Let(Var, Box<Exp>),
}

impl fmt::Debug for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exp::Call(subject, args) => {
                let mut tuple = f.debug_tuple("call");
                tuple.field(subject);
                for arg in args {
                    tuple.field(arg);
                }
                tuple.finish()
            }
            Exp::Seq(stmts, body) => {
                let mut tuple = f.debug_tuple("block");
                for stmt in stmts {
                    tuple.field(stmt);
                }
                tuple.field(body);
                tuple.finish()
            }
            Exp::Binop(lhs, op, rhs) => {
                let mut tuple = f.debug_tuple("op");
                tuple.field(op);
                tuple.field(lhs);
                tuple.field(rhs);
                tuple.finish()
            }
            Exp::If(test, conseq, alt) => {
                let mut tuple = f.debug_tuple("if");
                tuple.field(test);
                tuple.field(conseq);
                tuple.field(alt);
                tuple.finish()
            }
            Exp::Value(v) => v.fmt(f),
            Exp::Var(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Exp(e) => e.fmt(f),
            Stmt::Let(x, e) => write!(f, "(let {} = {:?})", x, e),
        }
    }
}
