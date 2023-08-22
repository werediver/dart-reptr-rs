use tiny_set::with_tiny_set;

use super::{ty::Type, Expr};

#[derive(PartialEq, Eq, Debug)]
pub struct Var<'s> {
    pub modifiers: VarModifierSet,
    pub var_type: Option<Type<'s>>,
    pub name: &'s str,
    pub initializer: Option<Expr<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum VarModifier {
    External,
    Static,
    Const,
    Final,
    Late,
    /// Can only be used before non-final instance fields.
    Covariant,
}
