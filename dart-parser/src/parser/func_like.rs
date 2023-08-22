use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success, value},
    error::{context, ContextError, ParseError},
    multi::{fold_many0, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    func_like::{
        Func, FuncBody, FuncBodyContent, FuncBodyModifier, FuncModifier, FuncModifierSet,
        FuncParam, FuncParamModifier, FuncParamModifierSet, FuncParams, FuncParamsExtra, Getter,
        Setter,
    },
    FuncLike, MaybeRequired,
};

use super::{
    common::spbr,
    expr::block,
    expr::expr,
    ty::{identifier, ty},
    type_params::type_params,
    PResult,
};

pub fn func_like<'s, E>(s: &'s str) -> PResult<FuncLike, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_like",
        alt((
            func.map(FuncLike::Func),
            getter.map(FuncLike::Getter),
            setter.map(FuncLike::Setter),
        )),
    )
    .parse(s)
}

fn func<'s, E>(s: &'s str) -> PResult<Func, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func",
        tuple((
            alt((
                terminated(func_modifier_set, spbr),
                success(FuncModifierSet::default()),
            )),
            // Return type
            terminated(ty, opt(spbr)),
            // Function name
            terminated(identifier, opt(spbr)),
            opt(terminated(type_params, opt(spbr))),
            terminated(func_params, opt(spbr)),
            alt((func_body.map(Some), tag(";").map(|_| None))),
        ))
        .map(
            |(modifiers, return_type, name, type_params, params, body)| Func {
                modifiers,
                return_type,
                name,
                type_params: type_params.unwrap_or(Vec::new()),
                params,
                body,
            },
        ),
    )
    .parse(s)
}

fn getter<'s, E>(s: &'s str) -> PResult<Getter, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "getter",
        tuple((
            alt((
                terminated(func_modifier_set, spbr),
                success(FuncModifierSet::default()),
            )),
            // Return type
            terminated(ty, opt(spbr)),
            terminated(tag("get"), spbr),
            // Getter name
            terminated(identifier, opt(spbr)),
            alt((func_body.map(Some), tag(";").map(|_| None))),
        ))
        .map(|(modifiers, return_type, _, name, body)| Getter {
            modifiers,
            return_type,
            name,
            body,
        }),
    )
    .parse(s)
}

fn setter<'s, E>(s: &'s str) -> PResult<Setter, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "setter",
        tuple((
            alt((
                terminated(func_modifier_set, spbr),
                success(FuncModifierSet::default()),
            )),
            // The return type of the setter must be 'void' or absent
            opt(terminated(tag("void"), opt(spbr))),
            terminated(tag("set"), spbr),
            // Setter name
            terminated(identifier, opt(spbr)),
            terminated(func_params, opt(spbr)),
            alt((func_body.map(Some), tag(";").map(|_| None))),
        ))
        .map(|(modifiers, _, _, name, params, body)| Setter {
            modifiers,
            name,
            params,
            body,
        }),
    )
    .parse(s)
}

fn func_modifier_set<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<FuncModifierSet, E> {
    let (s, modifier) = func_modifier(s)?;

    let modifiers = FuncModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, func_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn func_modifier<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<FuncModifier, E> {
    alt((
        value(FuncModifier::External, tag("external")),
        value(FuncModifier::Static, tag("static")),
    ))(s)
}

pub fn func_params<'s, E>(s: &'s str) -> PResult<FuncParams<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_params",
        preceded(
            pair(tag("("), opt(spbr)),
            cut(terminated(
                pair(
                    func_params_pos_req,
                    opt(alt((
                        func_params_pos_opt.map(FuncParamsExtra::PositionalOpt),
                        func_params_named.map(FuncParamsExtra::Named),
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

fn func_params_pos_req<'s, E>(s: &'s str) -> PResult<Vec<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    terminated(
        separated_list0(
            pair(tag(","), opt(spbr)),
            terminated(func_param_pos, opt(spbr)),
        ),
        opt(pair(tag(","), opt(spbr))),
    )(s)
}

fn func_params_pos_opt<'s, E>(s: &'s str) -> PResult<Vec<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("["), opt(spbr)),
        cut(terminated(
            separated_list0(
                pair(tag(","), opt(spbr)),
                terminated(func_param_pos, opt(spbr)),
            ),
            pair(opt(pair(tag(","), opt(spbr))), tag("]")),
        )),
    )(s)
}

fn func_param_pos<'s, E>(s: &'s str) -> PResult<FuncParam, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_param_pos",
        tuple((
            alt((
                terminated(func_param_modifier_set, spbr),
                success(FuncParamModifierSet::default()),
            )),
            opt(terminated(tag("var"), spbr)),
            alt((
                // A type followed by a name
                pair(
                    terminated(ty, opt(spbr)).map(Some),
                    terminated(identifier, opt(spbr)),
                ),
                // Just a name
                terminated(identifier, opt(spbr)).map(|id| (None, id)),
            )),
            // An initializer
            opt(preceded(
                pair(tag("="), opt(spbr)),
                cut(terminated(expr, opt(spbr))),
            )),
        ))
        .map(
            |(modifiers, _, (param_type, name), initializer)| FuncParam {
                modifiers,
                param_type,
                name,
                initializer,
            },
        ),
    )(s)
}

fn func_params_named<'s, E>(s: &'s str) -> PResult<Vec<MaybeRequired<FuncParam>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_params_named",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                separated_list0(
                    pair(tag(","), opt(spbr)),
                    terminated(func_param_named, opt(spbr)),
                ),
                tuple((opt(pair(tag(","), opt(spbr))), tag("}"))),
            )),
        ),
    )(s)
}

fn func_param_named<'s, E>(s: &'s str) -> PResult<MaybeRequired<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_param_named",
        tuple((
            opt(terminated(tag("required"), spbr)),
            alt((
                terminated(func_param_modifier_set, spbr),
                success(FuncParamModifierSet::default()),
            )),
            opt(terminated(tag("var"), spbr)),
            alt((
                // A type followed by a name
                pair(
                    terminated(ty, opt(spbr)).map(Some),
                    terminated(identifier, opt(spbr)),
                ),
                // Just a name
                terminated(identifier, opt(spbr)).map(|id| (None, id)),
            )),
            // An initializer
            opt(preceded(
                pair(tag("="), opt(spbr)),
                cut(terminated(expr, opt(spbr))),
            )),
        ))
        .map(|(req, modifiers, _, (param_type, name), initializer)| {
            MaybeRequired::new(
                req.is_some(),
                FuncParam {
                    modifiers,
                    param_type,
                    name,
                    initializer,
                },
            )
        }),
    )
    .parse(s)
}

fn func_param_modifier_set<'s, E: ParseError<&'s str>>(
    s: &'s str,
) -> PResult<FuncParamModifierSet, E> {
    let (s, modifier) = func_param_modifier(s)?;

    let modifiers = FuncParamModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, func_param_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn func_param_modifier<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<FuncParamModifier, E> {
    alt((
        value(FuncParamModifier::Covariant, tag("covariant")),
        value(FuncParamModifier::Final, tag("final")),
    ))(s)
}

fn func_body<'s, E>(s: &'s str) -> PResult<FuncBody, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    pair(
        opt(terminated(func_body_modifier, opt(spbr))),
        func_body_content,
    )
    .map(|(modifier, content)| FuncBody { modifier, content })
    .parse(s)
}

pub fn func_body_content<'s, E>(s: &'s str) -> PResult<FuncBodyContent, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        preceded(
            pair(tag("=>"), opt(spbr)),
            terminated(expr, pair(opt(spbr), tag(";"))),
        )
        .map(FuncBodyContent::Expr),
        block.map(FuncBodyContent::Block),
    ))(s)
}

fn func_body_modifier<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<FuncBodyModifier, E> {
    alt((
        value(
            FuncBodyModifier::SyncGenerator,
            tuple((tag("sync"), opt(spbr), tag("*"))),
        ),
        value(FuncBodyModifier::Async, tag("async")),
        value(
            FuncBodyModifier::AsyncGenerator,
            tuple((tag("async"), opt(spbr), tag("*"))),
        ),
    ))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{ty::Type, Expr, NotFuncType, TypeParam};

    use super::*;

    #[test]
    fn func_block_test() {
        assert_eq!(
            func::<VerboseError<_>>("void f<T extends Object?, U>() {}x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::default(),
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    name: "f",
                    type_params: vec![
                        TypeParam {
                            name: "T",
                            extends: Some(Type::NotFunc(NotFuncType {
                                name: "Object",
                                type_args: Vec::new(),
                                is_nullable: true
                            })),
                        },
                        TypeParam {
                            name: "U",
                            extends: None
                        }
                    ],
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: None,
                    },
                    body: Some(FuncBody {
                        modifier: None,
                        content: FuncBodyContent::Block("{}")
                    })
                }
            ))
        );
    }

    #[test]
    fn func_pos_params_test() {
        assert_eq!(
            func::<VerboseError<_>>(
                "void f(int x, final double? y, [final bool mystery_flag = false]) {}x"
            ),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::default(),
                    return_type: Type::NotFunc(NotFuncType::name("void")),
                    name: "f",
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: vec![
                            FuncParam {
                                modifiers: FuncParamModifierSet::default(),
                                param_type: Some(Type::NotFunc(NotFuncType::name("int"))),
                                name: "x",
                                initializer: None,
                            },
                            FuncParam {
                                modifiers: FuncParamModifierSet::from_iter([
                                    FuncParamModifier::Final
                                ]),
                                param_type: Some(Type::NotFunc(NotFuncType {
                                    name: "double",
                                    type_args: Vec::new(),
                                    is_nullable: true,
                                })),
                                name: "y",
                                initializer: None,
                            },
                        ],
                        extra: Some(FuncParamsExtra::PositionalOpt(vec![FuncParam {
                            modifiers: FuncParamModifierSet::from_iter([FuncParamModifier::Final]),
                            param_type: Some(Type::NotFunc(NotFuncType::name("bool"))),
                            name: "mystery_flag",
                            initializer: Some(Expr::Ident("false")),
                        }])),
                    },
                    body: Some(FuncBody {
                        modifier: None,
                        content: FuncBodyContent::Block("{}")
                    })
                }
            ))
        );
    }

    #[test]
    fn func_sync_gen_test() {
        assert_eq!(
            func::<VerboseError<_>>("Iterable<int> f() sync* { print('<_<'); yield 0; }x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::default(),
                    return_type: Type::NotFunc(NotFuncType {
                        name: "Iterable",
                        type_args: vec![NotFuncType::name("int")],
                        is_nullable: false
                    }),
                    name: "f",
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: None,
                    },
                    body: Some(FuncBody {
                        modifier: Some(FuncBodyModifier::SyncGenerator),
                        content: FuncBodyContent::Block("{ print('<_<'); yield 0; }")
                    })
                }
            ))
        );
    }

    #[test]
    fn func_expr_test() {
        assert_eq!(
            func::<VerboseError<_>>("static List<String> f() => const [\"abc\"];x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::from_iter([FuncModifier::Static]),
                    return_type: Type::NotFunc(NotFuncType {
                        name: "List",
                        type_args: vec![NotFuncType::name("String")],
                        is_nullable: false,
                    }),
                    name: "f",
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: None,
                    },
                    body: Some(FuncBody {
                        modifier: None,
                        content: FuncBodyContent::Expr(Expr::Verbatim("const [\"abc\"]"))
                    })
                }
            ))
        );
    }

    #[test]
    fn func_expr_async_test() {
        assert_eq!(
            func::<VerboseError<_>>("static Future<String> f() async => \"abc\";x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::from_iter([FuncModifier::Static]),
                    return_type: Type::NotFunc(NotFuncType {
                        name: "Future",
                        type_args: vec![NotFuncType::name("String")],
                        is_nullable: false,
                    }),
                    name: "f",
                    type_params: Vec::new(),
                    params: FuncParams {
                        positional_req: Vec::new(),
                        extra: None,
                    },
                    body: Some(FuncBody {
                        modifier: Some(FuncBodyModifier::Async),
                        content: FuncBodyContent::Expr(Expr::String("abc"))
                    })
                }
            ))
        );
    }

    #[test]
    fn func_modifier_set_test() {
        assert_eq!(
            func_modifier_set::<VerboseError<_>>("external static "),
            Ok((
                " ",
                FuncModifierSet::from_iter([FuncModifier::External, FuncModifier::Static])
            ))
        );
    }

    #[test]
    fn setter_test() {
        assert_eq!(
            setter::<VerboseError<_>>("set name(String value) {} "),
            Ok((
                " ",
                Setter {
                    modifiers: FuncModifierSet::default(),
                    name: "name",
                    params: FuncParams {
                        positional_req: vec![FuncParam {
                            name: "value",
                            modifiers: FuncParamModifierSet::default(),
                            param_type: Some(Type::NotFunc(NotFuncType::name("String"))),
                            initializer: None
                        }],
                        extra: None
                    },
                    body: Some(FuncBody {
                        modifier: None,
                        content: FuncBodyContent::Block("{}")
                    })
                }
            ))
        );
    }
}
