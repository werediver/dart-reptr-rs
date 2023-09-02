use super::{class::ClassMember, func_call::FuncArg, NotFuncType, WithMeta};

#[derive(PartialEq, Eq, Debug)]
pub struct EnumTy<'s> {
    pub name: &'s str,
    pub implements: Vec<NotFuncType<'s>>,
    pub values: Vec<WithMeta<'s, EnumValue<'s>>>,
    pub members: Vec<WithMeta<'s, ClassMember<'s>>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct EnumValue<'s> {
    pub name: &'s str,
    pub args: Vec<FuncArg<'s>>,
}
