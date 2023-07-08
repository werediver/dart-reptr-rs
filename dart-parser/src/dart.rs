mod class_member_modifier;
mod class_modifier;

pub use class_member_modifier::*;
pub use class_modifier::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Verbatim(&'s str),
    Class(Class<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub body: &'s str,
}
