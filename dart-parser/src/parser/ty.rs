use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::{cut, fail, opt, recognize, success},
    error::{context, ContextError, ParseError},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    func_like::{FuncParams, FuncParamsExtra},
    ty::{FuncType, FuncTypeParamNamed, FuncTypeParamPos, Type},
    MaybeRequired, NotFuncType, TypeParam, WithMeta,
};

use super::{
    common::{sep_list, spbr, SepMode},
    meta::with_meta,
    type_params::type_params,
    PResult,
};

pub fn identifier<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    // Based on [Keywords](https://dart.dev/language/keywords).
    const RESERVED: [&str; 26] = [
        "assert", "break", "case", "catch", "class", "const", "continue", "default", "do", "else",
        "enum", "extends", "finally", "for", "if", "in", "is", "new", "rethrow", "return",
        "switch", "throw", "try", "var", "while",
        "with",
        // It is desirable to recognize the following words as identifiers
        // "false", "null", "super", "this", "true", "void",
    ];

    fn is_start_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }

    fn is_part_char(c: char) -> bool {
        is_start_char(c) || c.is_ascii_digit()
        // Parse composite identifiers as a single identifier
        || c == '.'
    }

    context("identifier", |s| {
        let (tail, id) = recognize(pair(
            take_while_m_n(1, 1, is_start_char),
            take_while(is_part_char),
        ))(s)?;

        if RESERVED.contains(&id) {
            fail(s)
        } else {
            Ok((tail, id))
        }
    })(s)
}

pub fn ty<'s, E>(s: &'s str) -> PResult<Type, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "ty",
        alt((
            func_type.map(Type::Func),
            not_func_type.map(Type::NotFunc),
            tuple_ty.map(Type::Tuple),
        )),
    )(s)
}

/// Parse an identifier with type arguments and the nullability indicator (e.g. `x`, `Future<int>?`).
pub fn not_func_type<'s, E>(s: &'s str) -> PResult<NotFuncType, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "not_func_type",
        tuple((
            identifier,
            opt(preceded(opt(spbr), type_args)),
            opt(preceded(opt(spbr), tag("?"))),
        ))
        .map(|(name, args, nullability_ind)| NotFuncType {
            name,
            type_args: args.unwrap_or(Vec::default()),
            is_nullable: nullability_ind.is_some(),
        }),
    )
    .parse(s)
}

pub fn type_args<'s, E>(s: &'s str) -> PResult<Vec<Type>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "type_args",
        preceded(
            pair(tag("<"), opt(spbr)),
            cut(terminated(
                separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), ty),
                pair(opt(spbr), tag(">")),
            )),
        ),
    )(s)
}

fn func_type<'s, E>(s: &'s str) -> PResult<Box<FuncType>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context("func_type", |s| {
        let (s, (return_type, fn_chain)) = tuple((
            opt(terminated(not_func_type, opt(spbr))),
            sep_list(
                1,
                SepMode::NoTrailing,
                opt(spbr),
                tuple((
                    preceded(
                        pair(tag("Function"), opt(spbr)),
                        alt((
                            terminated(type_params, opt(spbr)),
                            success(()).map(|_| Vec::new()),
                        )),
                    ),
                    func_type_params,
                    alt((preceded(opt(spbr), tag("?")).map(|_| true), success(false))),
                )),
            ),
        ))(s)?;

        if let Some(fn_type) = build_func_type(return_type, fn_chain) {
            Ok((s, fn_type))
        } else {
            fail(s)
        }
    })(s)
}

fn build_func_type<'s>(
    return_type: Option<NotFuncType<'s>>,
    fn_chain: Vec<(
        Vec<TypeParam<'s>>,
        FuncParams<'s, FuncTypeParamPos<'s>, FuncTypeParamNamed<'s>>,
        bool,
    )>,
) -> Option<Box<FuncType<'s>>> {
    let ty = fn_chain.into_iter().fold(
        Type::NotFunc(return_type.unwrap_or(NotFuncType::dynamic())),
        |ty, (type_params, fn_params, is_nullable)| {
            Type::func(FuncType {
                return_type: ty,
                type_params,
                params: fn_params,
                is_nullable,
            })
        },
    );

    match ty {
        Type::Func(fn_type) => Some(fn_type),
        Type::NotFunc(_) => None,
        Type::Tuple(_) => None,
    }
}

fn func_type_params<'s, E>(
    s: &'s str,
) -> PResult<FuncParams<FuncTypeParamPos<'s>, FuncTypeParamNamed<'s>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_type_params",
        preceded(
            pair(tag("("), opt(spbr)),
            cut(terminated(
                pair(
                    func_type_params_pos_req,
                    opt(alt((
                        func_type_params_pos_opt.map(FuncParamsExtra::PositionalOpt),
                        func_type_params_named.map(FuncParamsExtra::Named),
                    ))),
                ),
                pair(opt(spbr), tag(")")),
            )),
        )
        .map(|(positional_req, extra)| FuncParams {
            positional_req,
            extra,
        }),
    )
    .parse(s)
}

fn func_type_params_pos_req<'s, E>(s: &'s str) -> PResult<Vec<WithMeta<FuncTypeParamPos>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    sep_list(
        0,
        SepMode::AllowTrailing,
        pair(tag(","), opt(spbr)),
        terminated(with_meta(func_type_param_pos), opt(spbr)),
    )(s)
}

fn func_type_params_pos_opt<'s, E>(s: &'s str) -> PResult<Vec<WithMeta<FuncTypeParamPos>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("["), opt(spbr)),
        cut(terminated(
            sep_list(
                0,
                SepMode::AllowTrailing,
                pair(tag(","), opt(spbr)),
                terminated(with_meta(func_type_param_pos), opt(spbr)),
            ),
            tag("]"),
        )),
    )(s)
}

fn func_type_param_pos<'s, E>(s: &'s str) -> PResult<FuncTypeParamPos, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_param_pos",
        alt((
            // A type followed by a name
            pair(
                terminated(ty, opt(spbr)),
                terminated(identifier, opt(spbr)).map(Some),
            ),
            // Just a type
            terminated(ty, opt(spbr)).map(|ty| (ty, None)),
        ))
        .map(|(param_type, name)| FuncTypeParamPos { param_type, name }),
    )(s)
}

fn func_type_params_named<'s, E>(
    s: &'s str,
) -> PResult<Vec<WithMeta<MaybeRequired<FuncTypeParamNamed>>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_params_named",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                sep_list(
                    0,
                    SepMode::AllowTrailing,
                    pair(tag(","), opt(spbr)),
                    terminated(with_meta(func_type_param_named), opt(spbr)),
                ),
                tag("}"),
            )),
        ),
    )(s)
}

fn func_type_param_named<'s, E>(s: &'s str) -> PResult<MaybeRequired<FuncTypeParamNamed>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_param_named",
        tuple((
            opt(terminated(tag("required"), spbr)),
            // A type followed by a name
            terminated(ty, opt(spbr)),
            terminated(identifier, opt(spbr)),
        ))
        .map(|(req, param_type, name)| {
            MaybeRequired::new(req.is_some(), FuncTypeParamNamed { param_type, name })
        }),
    )
    .parse(s)
}

fn tuple_ty<'s, E>(s: &'s str) -> PResult<Vec<Type>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "tuple",
        preceded(
            pair(tag("("), opt(spbr)),
            terminated(
                sep_list(
                    0,
                    SepMode::AllowTrailing,
                    pair(tag(","), opt(spbr)),
                    terminated(ty, opt(spbr)),
                ),
                tag(")"),
            ),
        ),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{
        func_like::FuncParams,
        ty::{FuncTypeParamPos, Type},
    };

    use super::*;

    #[test]
    fn type_args_test() {
        assert_eq!(
            type_args::<VerboseError<_>>("<void Function()>x"),
            Ok((
                "x",
                vec![Type::func(FuncType {
                    return_type: Type::NotFunc(NotFuncType::void()),
                    type_params: Vec::new(),
                    params: FuncParams::default(),
                    is_nullable: false,
                })]
            ))
        );
    }

    #[test]
    fn type_generic_test() {
        assert_eq!(
            not_func_type::<VerboseError<_>>("Map<String, Object>? "),
            Ok((
                " ",
                NotFuncType {
                    name: "Map",
                    type_args: vec![
                        Type::NotFunc(NotFuncType::name("String")),
                        Type::NotFunc(NotFuncType::name("Object")),
                    ],
                    is_nullable: true,
                }
            ))
        );
    }

    #[test]
    fn type_generic_nested_test() {
        assert_eq!(
            not_func_type::<VerboseError<_>>("Map<String, List<int>> "),
            Ok((
                " ",
                NotFuncType {
                    name: "Map",
                    type_args: vec![
                        Type::NotFunc(NotFuncType::name("String")),
                        Type::NotFunc(NotFuncType {
                            name: "List",
                            type_args: vec![Type::NotFunc(NotFuncType::name("int"))],
                            is_nullable: false,
                        })
                    ],
                    is_nullable: false,
                }
            ))
        );
    }

    #[test]
    fn func_type_simple() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function()x"),
            Ok((
                "x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    type_params: Vec::new(),
                    params: FuncParams::default(),
                    is_nullable: false,
                })
            ))
        );
    }

    #[test]
    fn func_type_arg_type() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function(int) x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: vec![WithMeta::value(FuncTypeParamPos {
                            param_type: Type::NotFunc(NotFuncType::name("int")),
                            name: None
                        })],
                        extra: None,
                    },
                    is_nullable: false,
                })
            ))
        );
    }

    #[test]
    fn func_type_generic_type() {
        assert_eq!(
            func_type::<VerboseError<_>>("T Function<T>(T) x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("T")),
                    type_params: vec![TypeParam {
                        name: "T",
                        extends: None
                    }],
                    params: FuncParams {
                        positional_req: vec![WithMeta::value(FuncTypeParamPos {
                            param_type: Type::NotFunc(NotFuncType::name("T")),
                            name: None
                        })],
                        extra: None,
                    },
                    is_nullable: false,
                })
            ))
        );
    }

    #[test]
    fn func_type_arg_name() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function(int x)? x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: vec![WithMeta::value(FuncTypeParamPos {
                            param_type: Type::NotFunc(NotFuncType::name("int")),
                            name: Some("x"),
                        })],
                        extra: None,
                    },
                    is_nullable: true
                })
            ))
        );
    }

    #[test]
    fn func_type_arg_named() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function({int? x})? x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: Some(FuncParamsExtra::Named(vec![WithMeta::value(
                            MaybeRequired::new(
                                false,
                                FuncTypeParamNamed {
                                    param_type: Type::NotFunc(NotFuncType {
                                        name: "int",
                                        type_args: Vec::new(),
                                        is_nullable: true
                                    }),
                                    name: "x",
                                }
                            )
                        )])),
                    },
                    is_nullable: true
                })
            ))
        );
    }

    #[test]
    fn func_type_arg_named_longer() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function({List<void Function()>? funcs})? x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: Some(FuncParamsExtra::Named(vec![WithMeta::value(
                            MaybeRequired::new(
                                false,
                                FuncTypeParamNamed {
                                    param_type: Type::NotFunc(NotFuncType {
                                        name: "List",
                                        type_args: vec![Type::func(FuncType {
                                            return_type: Type::NotFunc(NotFuncType::void()),
                                            type_params: Vec::new(),
                                            params: FuncParams::default(),
                                            is_nullable: false,
                                        })],
                                        is_nullable: true
                                    }),
                                    name: "funcs",
                                }
                            )
                        )])),
                    },
                    is_nullable: true
                })
            ))
        );
    }

    #[test]
    fn func_type_inception() {
        assert_eq!(
            func_type::<VerboseError<_>>("void Function() Function()? Function() x"),
            Ok((
                " x",
                Box::new(FuncType {
                    return_type: Type::func(FuncType {
                        return_type: Type::func(FuncType {
                            return_type: Type::NotFunc(NotFuncType::name("void")),
                            type_params: Vec::new(),
                            params: FuncParams::default(),
                            is_nullable: false,
                        }),
                        type_params: Vec::new(),
                        params: FuncParams::default(),
                        is_nullable: true,
                    }),
                    type_params: Vec::new(),
                    params: FuncParams::default(),
                    is_nullable: false,
                })
            ))
        );
    }
}
