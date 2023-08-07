use tiny_set::with_tiny_set;

use super::{
    func::{FuncBodyContent, FuncParams},
    Comment, Func, IdentifierExt, Var,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub extends: Option<IdentifierExt<'s>>,
    pub implements: Vec<IdentifierExt<'s>>,
    pub body: Vec<ClassMember<'s>>,
}

/// The possible combinations are:
///
/// - `mixin` or `class`
/// - `mixin class`
/// - `base mixin` (only the `base` modifier can appear before a mixin declaration)
/// - `class` with other modifiers
///
/// See [Class modifiers](https://dart.dev/language/class-modifiers).
#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum ClassModifier {
    Class,
    Mixin,
    Abstract,
    Base,
    Final,
    Interface,
    Sealed,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ClassMember<'s> {
    Comment(Comment<'s>),
    Constructor(Constructor<'s>),
    Var(Var<'s>),
    Func(Func<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Constructor<'s> {
    pub modifier: Option<ConstructorModifier>,
    pub name: &'s str,
    pub params: FuncParams<'s>,
    pub body: Option<FuncBodyContent<'s>>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ConstructorModifier {
    Const,
    Factory,
}
