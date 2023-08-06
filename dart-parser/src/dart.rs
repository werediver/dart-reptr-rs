pub mod class;
pub mod comment;
pub mod directive;
pub mod enum_ty;
pub mod func;
pub mod var;

pub use class::Class;
pub use comment::Comment;
pub use directive::Directive;
pub use enum_ty::EnumTy;
pub use func::Func;
pub use var::Var;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Comment(Comment<'s>),
    Directive(Directive<'s>),
    Var(Var<'s>),
    Func(Func<'s>),
    Class(Class<'s>),
    Enum(EnumTy<'s>),
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
