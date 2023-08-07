use super::{Expr, IdentifierExt};

#[derive(PartialEq, Eq, Debug)]
pub struct FuncCall<'s> {
    pub ident: IdentifierExt<'s>,
    pub args: Vec<FuncArg<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncArg<'s> {
    pub name: Option<&'s str>,
    pub value: Expr<'s>,
}
