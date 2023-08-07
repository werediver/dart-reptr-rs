use super::FuncCall;

#[derive(PartialEq, Eq, Debug)]
pub enum Annotation<'s> {
    Ident(&'s str),
    FuncCall(FuncCall<'s>),
}
