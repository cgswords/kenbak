use pretty::{Doc, RcDoc};

use crate::shared::{
    ast::{Op, Triv, Var},
    ToDoc,
};

#[derive(Debug, Clone)]
pub enum Exp {
    Call(Triv, Vec<Triv>),
    Seq(Vec<Stmt>, Box<Exp>),
    If(Triv, Box<Exp>, Box<Exp>),
    Triv(Triv),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    LetBinop(Var, Triv, Op, Triv),
    LetCall(Var, Triv, Vec<Triv>),
    Let(Var, Triv),
    If(Triv, Vec<Stmt>, Vec<Stmt>),
    Call(Triv, Vec<Triv>),
}

impl ToDoc for Exp {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
            Exp::Call(subject, args) => {
                let args = [subject.to_doc()]
                    .into_iter()
                    .chain(args.iter().map(|arg| arg.to_doc()));
                RcDoc::text("(")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(1).group())
                    .append(RcDoc::text(")"))
            }
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
            Exp::Triv(t) => t.to_doc(),
        }
    }
}

impl ToDoc for Stmt {
    fn to_doc(&self) -> RcDoc<()> {
        match self {
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
            Stmt::LetCall(x, subject, args) => {
                let args = [RcDoc::text(x), subject.to_doc()].into_iter().chain(args.iter().map(|arg| arg.to_doc()));
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
            Stmt::Call(subject, args) => {
                let args = [subject.to_doc()]
                    .into_iter()
                    .chain(args.iter().map(|arg| arg.to_doc()));
                RcDoc::text("(")
                    .append(RcDoc::intersperse(args, Doc::line()).nest(1).group())
                    .append(RcDoc::text(")"))
            }
        }
    }
}
