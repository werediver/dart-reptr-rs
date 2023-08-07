pub mod annotation;
pub mod class;
pub mod comment;
pub mod directive;
pub mod enum_ty;
pub mod expr;
pub mod func;
pub mod func_call;
pub mod identifier_ext;
pub mod type_param;
pub mod var;

pub use annotation::Annotation;
pub use class::Class;
pub use comment::Comment;
pub use directive::Directive;
pub use enum_ty::EnumTy;
pub use expr::Expr;
pub use func::Func;
pub use func_call::FuncCall;
pub use identifier_ext::IdentifierExt;
pub use type_param::TypeParam;
pub use var::Var;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Comment(Comment<'s>),
    Directive(Directive<'s>),
    Annotation(Annotation<'s>),
    Var(Var<'s>),
    Func(Func<'s>),
    Class(Class<'s>),
    Enum(EnumTy<'s>),
}
