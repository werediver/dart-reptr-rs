use super::{Annotation, Comment, FuncLike, IdentifierExt, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub struct Extension<'s> {
    pub name: Option<&'s str>,
    pub type_params: Vec<TypeParam<'s>>,
    pub on: IdentifierExt<'s>,
    pub body: Vec<ExtensionMember<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ExtensionMember<'s> {
    Comment(Comment<'s>),
    Annotation(Annotation<'s>),
    FuncLike(FuncLike<'s>),
}