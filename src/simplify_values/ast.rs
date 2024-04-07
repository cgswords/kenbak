use crate::shared::ast::{Op, Triv, Value, Var};

#[derive(Debug, Clone)]
pub enum Exp {
    Call(Triv, Vec<Triv>),
    Binop(Var, Op, Triv),
    Triv(Triv),
    Seq(Vec<Stmt>, Box<Exp>),
    If(Box<Pred>, Box<Exp>, Box<Exp>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    If(Box<Pred>, Vec<Stmt>, Vec<Stmt>),
    Seq(Vec<Stmt>),
    Let(Var, Box<Exp>),
    Exp(Box<Exp>),
}

#[derive(Debug, Clone)]
pub enum Pred {
    Call(Triv, Vec<Triv>),
    Relop(Var, Op, Triv),
    Triv(Triv),
    Seq(Vec<Stmt>, Box<Pred>),
    If(Box<Pred>, Box<Pred>, Box<Pred>),
    True,
    False,
}
