use pretty::{Doc, RcDoc};

use crate::shared::{
    ast::{Op, Triv, Var},
    ToDoc,
};

#[derive(Debug, Clone)]
pub enum Exp {
    Call(Triv),
    Seq(Vec<Stmt>, Box<Exp>),
    If(Triv, Box<Exp>, Box<Exp>),
    Return,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Exp(Box<Exp>),
    LetBinop(Var, Triv, Op, Triv),
    Let(Var, Triv),
    If(Triv, Vec<Stmt>, Vec<Stmt>),
    Call(Triv),
    Push(Triv),
    ReturnSet(Triv),
}

impl ToDoc for Exp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Exp::Call(subject) => RcDoc::text("(")
                .append(subject.to_doc())
                .append(RcDoc::text(")")),
            Exp::Seq(stmts, last) => {
                let args = stmts
                    .iter()
                    .map(|stmt| stmt.to_doc())
                    .chain([last.to_doc()].into_iter());
                RcDoc::text("(begin ")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(2).group())
                    .append(RcDoc::text(")"))
            }
            Exp::If(test, conseq, alt) => {
                let args = [test.to_doc(), conseq.to_doc(), alt.to_doc()];
                RcDoc::text("(if ")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(2).group())
                    .append(RcDoc::text(")"))
            }
            Exp::Return => RcDoc::text("(return)"),
        }
    }
}

impl ToDoc for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Stmt::Exp(e) => e.to_doc(),
            Stmt::LetBinop(x, lhs, op, rhs) => {
                let binop = RcDoc::intersperse(
                    [RcDoc::text(format!("{:?}", op)), lhs.to_doc(), rhs.to_doc()],
                    Doc::line(),
                )
                .group();
                let args = [
                    RcDoc::text(x),
                    RcDoc::text("(")
                        .append(binop)
                        .append(RcDoc::text(")"))
                        .group(),
                ];
                RcDoc::text("(set! ")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(2).group())
                    .append(RcDoc::text(")"))
            }
            Stmt::Let(x, e) => {
                let args = [RcDoc::text(x), e.to_doc()];
                RcDoc::text("(set! ")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(2).group())
                    .append(RcDoc::text(")"))
            }
            Stmt::If(test, conseq, alt) => {
                let args = [test.to_doc(),
                    RcDoc::intersperse(conseq.iter().map(|stmt| stmt.to_doc()), Doc::line()).group(),
                    RcDoc::intersperse(alt.iter().map(|stmt| stmt.to_doc()), Doc::line()).group()];
                RcDoc::text("(if ")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(2).group())
                    .append(RcDoc::text(")"))
            },
            Stmt::Call(t) => RcDoc::text("(")
                .append(t.to_doc())
                .append(RcDoc::text(")")),
            Stmt::Push(t) => RcDoc::text("(push! ")
                .append(t.to_doc())
                .append(RcDoc::text(")")),
            Stmt::ReturnSet(t) => RcDoc::text("(return-set! ")
                .append(t.to_doc())
                .append(RcDoc::text(")")),
        }
    }
}
