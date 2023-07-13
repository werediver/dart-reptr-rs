mod class_member_modifier;
mod class_modifier;

pub use class_member_modifier::*;
pub use class_modifier::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Verbatim(&'s str),
    Import(Import<'s>),
    Class(Class<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Import<'s> {
    pub target: &'s str,
    // pub prefix: &str,
    // pub show: Vec<&str>,
    // pub hide: Vec<&str>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Class<'s> {
    pub modifiers: ClassModifierSet,
    pub name: &'s str,
    pub body: &'s str,
}
