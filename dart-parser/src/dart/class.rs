use tiny_set::with_tiny_set;

use super::{
    func_like::{FuncBodyContent, FuncParam, FuncParams},
    Annotation, Comment, FuncLike, NotFuncType, TypeParam, Var,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub type_params: Vec<TypeParam<'s>>,
    /// The base class.
    pub extends: Option<NotFuncType<'s>>,
    /// Mix-ins.
    pub with: Vec<NotFuncType<'s>>,
    /// Interfaces.
    pub implements: Vec<NotFuncType<'s>>,
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
    pub params: FuncParams<FuncParam<'s>>,
    pub body: Option<FuncBodyContent<'s>>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ConstructorModifier {
    Const,
    Factory,
    External,
}
