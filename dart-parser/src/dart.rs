pub mod class;
pub mod comment;
pub mod directive;
pub mod enum_ty;
pub mod func;
pub mod identifier_ext;
pub mod var;

pub use class::Class;
pub use comment::Comment;
pub use directive::Directive;
pub use enum_ty::EnumTy;
pub use func::Func;
pub use identifier_ext::IdentifierExt;
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
