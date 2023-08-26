use super::{func_like::FuncParams, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub enum Type<'s> {
    NotFunc(NotFuncType<'s>),
    Func(Box<FuncType<'s>>),
}

impl<'s> Type<'s> {
    pub fn func(func_type: FuncType<'s>) -> Self {
        Self::Func(Box::new(func_type))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct NotFuncType<'s> {
    pub name: &'s str,
    pub type_args: Vec<Type<'s>>,
    pub is_nullable: bool,
}

impl<'s> NotFuncType<'s> {
    pub fn name(name: &'s str) -> Self {
        Self {
            name,
            type_args: Vec::default(),
            is_nullable: false,
        }
    }

    pub fn void() -> Self {
        Self {
            name: "void",
            type_args: Vec::new(),
            is_nullable: false,
        }
    }

    pub fn dynamic() -> Self {
        Self {
            name: "dynamic",
            type_args: Vec::new(),
            is_nullable: false,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncType<'s> {
    pub return_type: Type<'s>,
    pub type_params: Vec<TypeParam<'s>>,
    pub params: FuncParams<FuncTypeParam<'s>>,
    pub is_nullable: bool,
}

/// A parameter in a function type.
#[derive(PartialEq, Eq, Debug)]
pub struct FuncTypeParam<'s> {
    pub param_type: Type<'s>,
    pub name: Option<&'s str>,
}
