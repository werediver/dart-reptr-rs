use super::IdentifierExt;

#[derive(PartialEq, Eq, Debug)]
pub struct EnumTy<'s> {
    pub name: &'s str,
    pub implements: Vec<IdentifierExt<'s>>,
}
