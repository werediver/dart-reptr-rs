use super::{ty::Type, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub struct TypeDef<'s> {
    pub name: &'s str,
    pub type_params: Vec<TypeParam<'s>>,
    pub target: Type<'s>,
}
