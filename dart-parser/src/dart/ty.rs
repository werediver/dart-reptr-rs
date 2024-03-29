use super::{func_like::FuncParams, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub enum Type<'s> {
    NotFunc(NotFuncType<'s>),
    Func(Box<FuncType<'s>>),
    Tuple(Tuple<'s>),
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
    pub params: FuncParams<'s, FuncTypeParamPos<'s>, FuncTypeParamNamed<'s>>,
    pub is_nullable: bool,
}

/// A positional parameter in a function type.
#[derive(PartialEq, Eq, Debug)]
pub struct FuncTypeParamPos<'s> {
    pub param_type: Type<'s>,
    pub name: Option<&'s str>,
}

/// A named parameter in a function type.
#[derive(PartialEq, Eq, Debug)]
pub struct FuncTypeParamNamed<'s> {
    pub param_type: Type<'s>,
    pub name: &'s str,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Tuple<'s> {
    /// A name can be specified for a positional tuple parameter,
    /// but has no meaning whatsoever.
    pub params_pos: Vec<FuncTypeParamPos<'s>>,
    pub params_named: Vec<FuncTypeParamNamed<'s>>,
    pub is_nullable: bool,
}
