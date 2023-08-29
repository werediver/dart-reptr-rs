use tiny_set::with_tiny_set;

use super::{ty::Type, Expr, MaybeRequired, TypeParam};

#[derive(PartialEq, Eq, Debug)]
pub enum FuncLike<'s> {
    Func(Func<'s>),
    Getter(Getter<'s>),
    Setter(Setter<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Func<'s> {
    pub modifiers: FuncModifierSet,
    pub return_type: Type<'s>,
    pub name: &'s str,
    pub type_params: Vec<TypeParam<'s>>,
    pub params: FuncParams<FuncParam<'s>>,
    pub body: Option<FuncBody<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Getter<'s> {
    pub modifiers: FuncModifierSet,
    pub return_type: Type<'s>,
    pub name: &'s str,
    pub body: Option<FuncBody<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Setter<'s> {
    pub modifiers: FuncModifierSet,
    pub name: &'s str,
    /// Setters must declare exactly one required positional parameter.
    pub params: FuncParams<FuncParam<'s>>,
    pub body: Option<FuncBody<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncModifier {
    External,
    Static,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncParams<ParamPos, ParamNamed = ParamPos> {
    pub positional_req: Vec<ParamPos>,
    pub extra: Option<FuncParamsExtra<ParamPos, ParamNamed>>,
}

impl<T, U> Default for FuncParams<T, U> {
    fn default() -> Self {
        Self {
            positional_req: Vec::new(),
            extra: None,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum FuncParamsExtra<ParamPos, ParamNamed = ParamPos> {
    PositionalOpt(Vec<ParamPos>),
    Named(Vec<MaybeRequired<ParamNamed>>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncParam<'s> {
    pub modifiers: FuncParamModifierSet,
    pub param_type: Option<Type<'s>>,
    pub name: &'s str,
    pub initializer: Option<Expr<'s>>,
}

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncParamModifier {
    /// Can only be used for parameters in instance methods.
    Covariant,
    Final,
}

#[derive(PartialEq, Eq, Debug)]
pub struct FuncBody<'s> {
    pub modifier: Option<FuncBodyModifier>,
    pub content: FuncBodyContent<'s>,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum FuncBodyModifier {
    SyncGenerator,
    Async,
    AsyncGenerator,
}

#[derive(PartialEq, Eq, Debug)]
pub enum FuncBodyContent<'s> {
    Block(&'s str),
    /// Not allowed in generator functions and constructors, except factory constructors.
    Expr(Expr<'s>),
}
