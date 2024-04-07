use std::fmt;
use crate::shared::ast::{Var, Op, Value};

#[derive(Debug, Clone)]
pub enum Exp {
    Call(Triv, Vec<Triv>),
    Seq(Vec<Stmt>, Box<Exp>),
    Binop(Triv, Op, Triv),
    If(Triv,  Box<Exp>, Box<Exp>),
    Triv(Triv),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Exp(Box<Exp>),
    Let(Var, Box<Exp>),
}

#[derive(Debug, Clone)]
pub enum Triv {
    Value(Value),
    Var(Var)
<Lua 265: ~/.local/share/nvim/lazy/nvim-cmp/lua/cmp/utils/keymap.lua:127>}


<Lua 142: ~/.local/share/nvim/lazy/nvim-cmp/lua/cmp/utils/keymap.lua:127>impl fmt::Display for Exp {


}
impl fmt::Debug for Exp {<Lua 142: ~/.local/share/nvim/lazy/nvim-cmp/lua/cmp/utils/keymap.lua:127>
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
            Exp::Triv(triv) => {
                write!(f, "{:?}", triv)
            }
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

impl fmt::Debug for Triv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Triv::Value(v) => v.fmt(f),
            Triv::Var(x) => write!(f, "{}", x),
        }
    }
}
