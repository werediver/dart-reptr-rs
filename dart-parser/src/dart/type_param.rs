use super::ty::Type;

#[derive(PartialEq, Eq, Debug)]
pub struct TypeParam<'s> {
    pub name: &'s str,
    pub extends: Option<Type<'s>>,
}
