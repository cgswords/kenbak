use crate::shared::ast::{Op, Value, Var};

#[derive(Debug, Clone)]
pub enum Exp {
    Call(Box<Exp>, Vec<Exp>),
    Seq(Vec<Stmt>, Box<Exp>),
    If(Box<Pred>, Box<Exp>, Box<Exp>),
    Binop(Box<Exp>, Op, Box<Exp>),
    Value(Value),
    Var(Var),
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
    Call(Box<Exp>, Vec<Exp>),
    Relop(Box<Exp>, Op, Box<Exp>),
    Seq(Vec<Stmt>, Box<Pred>),
    If(Box<Pred>, Box<Pred>, Box<Pred>),
    True,
    False,
}
