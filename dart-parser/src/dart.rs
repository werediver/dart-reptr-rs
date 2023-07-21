mod class;
pub mod directive;
mod member_modifier;

pub use class::*;
pub use member_modifier::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Verbatim(&'s str),
    Directive(directive::Directive<'s>),
    Class(Class<'s>),
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
