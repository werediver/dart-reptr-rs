use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, fail, opt, success, value},
    multi::{fold_many0, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    Func, FuncBody, FuncBodyContent, FuncBodyModifier, FuncBodyModifierSet, FuncModifier,
    FuncModifierSet,
};

use super::{
    common::{identifier, identifier_ext, spbr},
    expr::expr,
    scope::block,
    PResult,
};

pub fn func(s: &str) -> PResult<Func> {
    tuple((
        alt((
            terminated(func_modifier_set, spbr),
            success(FuncModifierSet::default()),
        )),
        identifier_ext,
        preceded(opt(spbr), identifier),
        opt(preceded(opt(spbr), type_params)),
        preceded(opt(spbr), func_params),
        alt((
            preceded(opt(spbr), func_body).map(Some),
            pair(opt(spbr), tag(";")).map(|_| None),
        )),
    ))
    .map(
        |(modifiers, return_type, name, _type_params, _params, body)| Func {
            modifiers,
            return_type,
            name,
            // type_params,
            // params,
            body,
        },
    )
    .parse(s)
}

fn func_modifier_set(s: &str) -> PResult<FuncModifierSet> {
    let (s, modifier) = func_modifier(s)?;

    let modifiers = FuncModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, func_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn func_modifier(s: &str) -> PResult<FuncModifier> {
    alt((
        value(FuncModifier::External, tag("external")),
        value(FuncModifier::Static, tag("static")),
    ))(s)
}

fn type_params(s: &str) -> PResult<Vec<()>> {
    preceded(
        pair(tag("<"), opt(spbr)),
        cut(terminated(
            separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), type_param),
            pair(opt(spbr), tag(">")),
        )),
    )(s)
}

fn type_param(s: &str) -> PResult<()> {
    fail(s)
}

fn func_params(s: &str) -> PResult<Vec<()>> {
    preceded(
        pair(tag("("), opt(spbr)),
        cut(terminated(
            separated_list0(tuple((opt(spbr), tag(","), opt(spbr))), func_param),
            pair(opt(spbr), tag(")")),
        )),
    )(s)
}

fn func_param(s: &str) -> PResult<()> {
    fail(s)
}

fn func_body(s: &str) -> PResult<FuncBody> {
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

fn func_body_content(s: &str) -> PResult<FuncBodyContent> {
    alt((
        preceded(
            pair(tag("=>"), opt(spbr)),
            terminated(expr, pair(opt(spbr), tag(";"))),
        )
        .map(FuncBodyContent::Expr),
        preceded(opt(spbr), block).map(FuncBodyContent::Block),
    ))(s)
}

fn func_body_modifiers(s: &str) -> PResult<FuncBodyModifierSet> {
    let (s, modifier) = func_body_modifier(s)?;

    let modifiers = FuncBodyModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, func_body_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn func_body_modifier(s: &str) -> PResult<FuncBodyModifier> {
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
    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn func_block_test() {
        assert_eq!(
            func("void f() {}x"),
            Ok((
                "x",
                Func {
                    modifiers: FuncModifierSet::default(),
                    return_type: IdentifierExt::name("void"),
                    name: "f",
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
            func("Iterable<int> f() sync* { print('<_<'); yield 0; }x"),
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
            func("static List<String> f() => const [\"abc\"];x"),
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
            func("static Future<String> f() async => \"abc\";x"),
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
            func_modifier_set("external static "),
            Ok((
                " ",
                FuncModifierSet::from_iter([FuncModifier::External, FuncModifier::Static])
            ))
        );
    }
}
