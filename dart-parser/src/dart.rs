pub mod annotation;
pub mod class;
pub mod comment;
pub mod directive;
pub mod enum_ty;
pub mod expr;
pub mod extension;
pub mod func_call;
pub mod func_like;
pub mod maybe_required;
pub mod ty;
pub mod type_param;
pub mod typedef;
pub mod var;

pub use annotation::Annotation;
pub use class::Class;
pub use comment::Comment;
pub use directive::Directive;
pub use enum_ty::EnumTy;
pub use expr::Expr;
pub use extension::Extension;
pub use func_call::FuncCall;
pub use func_like::FuncLike;
pub use maybe_required::MaybeRequired;
pub use ty::NotFuncType;
pub use type_param::TypeParam;
pub use typedef::TypeDef;
pub use var::Var;

#[derive(PartialEq, Eq, Debug)]
pub enum Dart<'s> {
    Comment(Comment<'s>),
    Directive(Directive<'s>),
    Annotation(Annotation<'s>),
    TypeDef(TypeDef<'s>),
    Var(Var<'s>),
    FuncLike(FuncLike<'s>),
    Class(Class<'s>),
    Enum(EnumTy<'s>),
    Extension(Extension<'s>),
}
