mod class;
pub mod directive;
mod func;
mod var;

pub use class::*;
pub use func::*;
pub use var::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Verbatim(&'s str),
    Directive(directive::Directive<'s>),
    Var(Var<'s>),
    Func(Func<'s>),
    Class(Class<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct IdentifierExt<'s> {
    pub name: &'s str,
    pub type_args: Vec<IdentifierExt<'s>>,
    pub is_nullable: bool,
}

impl<'s> IdentifierExt<'s> {
    pub fn name(name: &'s str) -> Self {
        Self {
            name,
            type_args: Vec::default(),
            is_nullable: false,
        }
    }
}
