use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, fail, opt, success, value},
    error::{context, ContextError, ParseError},
    multi::{fold_many0, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    func::{
        FuncBody, FuncBodyContent, FuncBodyModifier, FuncBodyModifierSet, FuncModifier,
        FuncModifierSet, FuncParam, FuncParamModifier, FuncParamModifierSet, FuncParams,
    },
    Func,
};

use super::{
    common::{identifier, identifier_ext, spbr},
    expr::expr,
    scope::block,
    PResult,
};

pub fn func<'s, E>(s: &'s str) -> PResult<Func, E>
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
            terminated(identifier_ext, opt(spbr)),
            // Function name
            terminated(identifier, opt(spbr)),
            opt(terminated(type_params, opt(spbr))),
            terminated(func_params, opt(spbr)),
            alt((func_body.map(Some), tag(";").map(|_| None))),
        ))
        .map(
            |(modifiers, return_type, name, _type_params, params, body)| Func {
                modifiers,
                return_type,
                name,
                // type_params,
                params,
                body,
            },
        ),
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

fn type_params<'s, E>(s: &'s str) -> PResult<Vec<()>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "type_params",
        preceded(
            pair(tag("<"), opt(spbr)),
            cut(terminated(
                separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), type_param),
                pair(opt(spbr), tag(">")),
            )),
        ),
    )(s)
}

fn type_param<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<(), E> {
    fail(s)
}

fn func_params<'s, E>(s: &'s str) -> PResult<FuncParams, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_params",
        preceded(
            pair(tag("("), opt(spbr)),
            cut(terminated(
                pair(
                    terminated(func_params_pos, opt(spbr)),
                    opt(terminated(func_params_named, opt(spbr))),
                ),
                pair(opt(spbr), tag(")")),
            )),
        )
        .map(|(positional, named)| FuncParams {
            positional,
            named: named.unwrap_or(Vec::new()),
        }),
    )
    .parse(s)
}

fn func_params_pos<'s, E>(s: &'s str) -> PResult<Vec<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    pair(
        // Required positional parameters
        terminated(
            separated_list0(
                tuple((opt(spbr), tag(","), opt(spbr))),
                func_param_pos(true),
            ),
            opt(tuple((opt(spbr), tag(","), opt(spbr)))),
        ),
        // Optional positional parameters
        opt(preceded(
            pair(tag("["), opt(spbr)),
            cut(terminated(
                separated_list0(
                    tuple((opt(spbr), tag(","), opt(spbr))),
                    func_param_pos(false),
                ),
                tuple((opt(spbr), opt(pair(tag(","), opt(spbr))), tag("]"))),
            )),
        )),
    )
    .map(|(mut req, opt)| {
        if let Some(mut opt) = opt {
            req.append(&mut opt);
        }

        req
    })
    .parse(s)
}

fn func_param_pos<'s, E>(is_required: bool) -> impl FnMut(&'s str) -> PResult<FuncParam, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    move |s| {
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
                        terminated(identifier_ext, opt(spbr)).map(Some),
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
                    is_required,
                    modifiers,
                    param_type,
                    name,
                    initializer,
                },
            ),
        )
        .parse(s)
    }
}

fn func_params_named<'s, E>(s: &'s str) -> PResult<Vec<FuncParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_params_named",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                separated_list0(tuple((opt(spbr), tag(","), opt(spbr))), func_param_named),
                tuple((opt(pair(tag(","), opt(spbr))), tag("}"))),
            )),
        ),
    )(s)
}

fn func_param_named<'s, E>(s: &'s str) -> PResult<FuncParam, E>
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
                    terminated(identifier_ext, opt(spbr)).map(Some),
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
            |(req, modifiers, _, (param_type, name), initializer)| FuncParam {
                is_required: req.is_some(),
                modifiers,
                param_type,
                name,
                initializer,
            },
        ),
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
        alt((
            terminated(func_body_modifiers, opt(spbr)),
            success(FuncBodyModifierSet::default()),
        )),
        func_body_content,
    )
    .map(|(modifiers, content)| FuncBody { modifiers, content })
    .parse(s)
}

fn func_body_content<'s, E>(s: &'s str) -> PResult<FuncBodyContent, E>
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

fn func_body_modifiers<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<FuncBodyModifierSet, E> {
    let (s, modifier) = func_body_modifier(s)?;

    let modifiers = FuncBodyModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, func_body_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
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

    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn func_block_test() {
        assert_eq!(
            func::<VerboseError<_>>("void f() {}x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::default(),
                    return_type: IdentifierExt::name("void"),
                    name: "f",
                    params: FuncParams {
                        positional: Vec::new(),
                        named: Vec::new(),
                    },
                    body: Some(FuncBody {
                        modifiers: FuncBodyModifierSet::default(),
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
                    return_type: IdentifierExt::name("void"),
                    name: "f",
                    params: FuncParams {
                        positional: vec![
                            FuncParam {
                                is_required: true,
                                modifiers: FuncParamModifierSet::default(),
                                param_type: Some(IdentifierExt::name("int")),
                                name: "x",
                                initializer: None,
                            },
                            FuncParam {
                                is_required: true,
                                modifiers: FuncParamModifierSet::from_iter([
                                    FuncParamModifier::Final
                                ]),
                                param_type: Some(IdentifierExt {
                                    name: "double",
                                    type_args: Vec::new(),
                                    is_nullable: true,
                                }),
                                name: "y",
                                initializer: None,
                            },
                            FuncParam {
                                is_required: false,
                                modifiers: FuncParamModifierSet::from_iter([
                                    FuncParamModifier::Final
                                ]),
                                param_type: Some(IdentifierExt::name("bool",)),
                                name: "mystery_flag",
                                initializer: Some("false"),
                            }
                        ],
                        named: Vec::new(),
                    },
                    body: Some(FuncBody {
                        modifiers: FuncBodyModifierSet::default(),
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
                    return_type: IdentifierExt {
                        name: "Iterable",
                        type_args: vec![IdentifierExt::name("int")],
                        is_nullable: false
                    },
                    name: "f",
                    params: FuncParams {
                        positional: Vec::new(),
                        named: Vec::new(),
                    },
                    body: Some(FuncBody {
                        modifiers: FuncBodyModifierSet::from_iter([
                            FuncBodyModifier::SyncGenerator
                        ]),
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
                    return_type: IdentifierExt {
                        name: "List",
                        type_args: vec![IdentifierExt::name("String")],
                        is_nullable: false,
                    },
                    name: "f",
                    params: FuncParams {
                        positional: Vec::new(),
                        named: Vec::new(),
                    },
                    body: Some(FuncBody {
                        modifiers: FuncBodyModifierSet::default(),
                        content: FuncBodyContent::Expr("const [\"abc\"]")
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
                    return_type: IdentifierExt {
                        name: "Future",
                        type_args: vec![IdentifierExt::name("String")],
                        is_nullable: false,
                    },
                    name: "f",
                    params: FuncParams {
                        positional: Vec::new(),
                        named: Vec::new(),
                    },
                    body: Some(FuncBody {
                        modifiers: FuncBodyModifierSet::from_iter([FuncBodyModifier::Async]),
                        content: FuncBodyContent::Expr("\"abc\"")
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
}
