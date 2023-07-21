use tiny_set::with_tiny_set;

use super::IdentifierExt;

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub extends: Option<IdentifierExt<'s>>,
    pub implements: Vec<IdentifierExt<'s>>,
    pub body: &'s str,
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
