mod class_member_modifier;
mod class_modifier;
pub mod directive;

pub use class_member_modifier::*;
pub use class_modifier::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Verbatim(&'s str),
    Directive(directive::Directive<'s>),
    Class(Class<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub extends: Option<IdentifierExt<'s>>,
    pub implements: Vec<IdentifierExt<'s>>,
    pub body: &'s str,
}

#[derive(PartialEq, Eq, Debug)]
pub struct IdentifierExt<'s> {
    pub name: &'s str,
    pub type_args: Vec<IdentifierExt<'s>>,
}

impl<'s> IdentifierExt<'s> {
    pub fn name(name: &'s str) -> Self {
        Self {
            name,
            type_args: Vec::default(),
        }
    }
}
