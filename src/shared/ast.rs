use core::fmt;
use std::collections::BTreeMap;

use pretty::RcDoc;

use super::ToDoc;

#[derive(Debug, Clone)]
pub struct Program<Body> {
    pub funcs: BTreeMap<Var, Body>,
}

#[derive(Clone)]
pub enum Op {
    Add,
    Sub,
    Eq,
    Neq,
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Eq => write!(f, "=="),
            Op::Neq => write!(f, "!="),
        }
    }
}

pub type Var = String;

#[derive(Clone)]
pub enum Value {
    Int(u8),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Triv {
    Value(Value),
    Var(Var),
    Return,
}

impl ToDoc for Triv {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Triv::Value(n) => RcDoc::text(format!("{:?}", n)),
            Triv::Var(x) => RcDoc::text(x),
            Triv::Return => RcDoc::text("%ret"),
        }
    }
}
