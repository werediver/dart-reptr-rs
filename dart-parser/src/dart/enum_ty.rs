use super::{class::ClassMember, func_call::FuncArg, IdentifierExt};

#[derive(PartialEq, Eq, Debug)]
pub struct EnumTy<'s> {
    pub name: &'s str,
    pub implements: Vec<IdentifierExt<'s>>,
    pub values: Vec<EnumValue<'s>>,
    pub members: Vec<ClassMember<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct EnumValue<'s> {
    pub name: &'s str,
    pub args: Vec<FuncArg<'s>>,
}
