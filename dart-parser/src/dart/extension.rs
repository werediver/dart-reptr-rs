use super::{ty::Type, Annotation, Comment, FuncLike, TypeParam, Var};

#[derive(PartialEq, Eq, Debug)]
pub struct Extension<'s> {
    pub name: Option<&'s str>,
    pub type_params: Vec<TypeParam<'s>>,
    pub on: Type<'s>,
    pub body: Vec<ExtensionMember<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExtensionMember<'s> {
    Comment(Comment<'s>),
    Annotation(Annotation<'s>),
    FuncLike(FuncLike<'s>),
    /// Only static fields can be declared in extensions.
    Var(Var<'s>),
}
