use super::FuncCall;

/// An annotation must precede a declaration.
#[derive(PartialEq, Eq, Debug)]
pub enum Annotation<'s> {
    Ident(&'s str),
    /// Type arguments are not allowed in annotations.
    FuncCall(FuncCall<'s>),
}
