use tiny_set::with_tiny_set;

use super::{
    func_like::{FuncBodyContent, FuncParams},
    Annotation, Comment, FuncLike, IdentifierExt, TypeParam, Var,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub type_params: Vec<TypeParam<'s>>,
    /// The base class.
    pub extends: Option<IdentifierExt<'s>>,
    /// Mix-ins.
    pub with: Vec<IdentifierExt<'s>>,
    /// Interfaces.
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
    Annotation(Annotation<'s>),
    Constructor(Constructor<'s>),
    Var(Var<'s>),
    FuncLike(FuncLike<'s>),
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
    External,
}
