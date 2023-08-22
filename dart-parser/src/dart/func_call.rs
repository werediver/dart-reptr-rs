use super::{Expr, NotFuncType};

#[derive(PartialEq, Eq, Debug)]
pub struct FuncCall<'s> {
    pub ident: NotFuncType<'s>,
    pub args: Vec<FuncArg<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncArg<'s> {
    pub name: Option<&'s str>,
    pub value: Expr<'s>,
}
