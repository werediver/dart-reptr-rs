use tiny_set::with_tiny_set;

use super::IdentifierExt;

#[derive(PartialEq, Eq, Debug)]
pub struct Var<'s> {
    pub modifiers: VarModifierSet,
    pub var_type: Option<IdentifierExt<'s>>,
    pub name: &'s str,
    pub initializer: Option<&'s str>,
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
    Covariant,
}
