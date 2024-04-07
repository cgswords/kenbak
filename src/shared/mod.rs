use pretty::RcDoc;

pub mod ast;

pub trait ToDoc {
    fn to_doc(&self) -> RcDoc<()>;
}
