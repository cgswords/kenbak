use pretty::RcDoc;

pub mod ast;
pub mod registers;

pub trait ToDoc {
    fn to_doc(&self) -> RcDoc<()>;
}
