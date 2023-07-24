use tiny_set::with_tiny_set;

use super::IdentifierExt;

#[derive(PartialEq, Eq, Debug)]
pub struct Func<'s> {
    pub modifiers: FuncModifierSet,
    pub return_type: IdentifierExt<'s>,
    pub name: &'s str,
    // pub type_params: Vec<_>,
    // pub params: Vec<_>,
    pub body: Option<FuncBody<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncModifier {
    External,
    Static,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncBody<'s> {
    pub modifiers: FuncBodyModifierSet,
    pub content: FuncBodyContent<'s>,
}

#[with_tiny_set]
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
    /// Not allowed in generator functions.
    Expr(&'s str),
}
