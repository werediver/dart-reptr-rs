use super::IdentifierExt;

#[derive(PartialEq, Eq, Debug)]
pub struct TypeParam<'s> {
    pub name: &'s str,
    pub extends: Option<IdentifierExt<'s>>,
}
