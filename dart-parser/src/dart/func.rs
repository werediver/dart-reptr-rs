use tiny_set::with_tiny_set;

use super::{Expr, IdentifierExt, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub struct Func<'s> {
    pub modifiers: FuncModifierSet,
    pub return_type: IdentifierExt<'s>,
    pub name: &'s str,
    pub type_params: Vec<TypeParam<'s>>,
    pub params: FuncParams<'s>,
    pub body: Option<FuncBody<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncModifier {
    External,
    Static,
}

#[derive(PartialEq, Eq, Default, Debug)]
pub struct FuncParams<'s> {
    pub positional: Vec<FuncParam<'s>>,
    pub named: Vec<FuncParam<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncParam<'s> {
    pub is_required: bool,
    pub modifiers: FuncParamModifierSet,
    pub param_type: Option<IdentifierExt<'s>>,
    pub name: &'s str,
    pub initializer: Option<Expr<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncParamModifier {
    Covariant,
    Final,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncBody<'s> {
    pub modifier: Option<FuncBodyModifier>,
    pub content: FuncBodyContent<'s>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncBodyModifier {
    SyncGenerator,
    Async,
    AsyncGenerator,
}

#[derive(PartialEq, Eq, Debug)]
pub enum FuncBodyContent<'s> {
    Block(&'s str),
    /// Not allowed in generator functions and constructors, except factory constructors.
    Expr(Expr<'s>),
}
