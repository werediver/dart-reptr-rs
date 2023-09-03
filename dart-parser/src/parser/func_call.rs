use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ContextError, ParseError},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::func_call::{FuncArg, FuncCall};

use super::{
    common::{sep_list, spbr, spbrc, SepMode},
    expr::expr,
    ty::{identifier, not_func_type},
    PResult,
};

pub fn _func_call<'s, E>(s: &'s str) -> PResult<FuncCall, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_call",
        pair(terminated(not_func_type, opt(spbrc)), func_args)
            .map(|(ident, args)| FuncCall { ident, args }),
    )(s)
}

/// In function/constructor-call annotations whitespace is not allowed in front
/// of the parentheses grouping the call arguments.
///
/// When an annotation is applied to a function declaration, this helps
/// differentiating between the annotation arguments and the return type of the
/// annotated function (which may be a tuple).
pub fn annotation_func_call<'s, E>(s: &'s str) -> PResult<FuncCall, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_call",
        pair(not_func_type, func_args).map(|(ident, args)| FuncCall { ident, args }),
    )(s)
}

pub fn func_args<'s, E>(s: &'s str) -> PResult<Vec<FuncArg>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "func_args",
        preceded(
            pair(tag("("), opt(spbr)),
            cut(terminated(
                sep_list(
                    0,
                    SepMode::AllowTrailing,
                    pair(tag(","), opt(spbr)),
                    terminated(func_arg, opt(spbr)),
                ),
                tag(")"),
            )),
        ),
    )(s)
}

pub fn func_arg<'s, E>(s: &'s str) -> PResult<FuncArg, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        pair(
            terminated(identifier, tuple((opt(spbrc), tag(":"), opt(spbrc)))),
            expr,
        )
        .map(|(name, value)| FuncArg {
            name: Some(name),
            value,
        }),
        expr.map(|value| FuncArg { name: None, value }),
    ))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{func_call::FuncArg, ty::Type, Expr, NotFuncType};

    use super::*;

    #[test]
    fn func_call_simple_test() {
        assert_eq!(
            _func_call::<VerboseError<_>>("f() x"),
            Ok((
                " x",
                FuncCall {
                    ident: NotFuncType::name("f"),
                    args: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn func_call_mixed_test() {
        assert_eq!(
            _func_call::<VerboseError<_>>("f<int>(1, named: two, verbatim: 1 + 2) x"),
            Ok((
                " x",
                FuncCall {
                    ident: NotFuncType {
                        name: "f",
                        type_args: vec![Type::NotFunc(NotFuncType::name("int"))],
                        is_nullable: false
                    },
                    args: vec![
                        FuncArg {
                            name: None,
                            value: Expr::Verbatim("1")
                        },
                        FuncArg {
                            name: Some("named"),
                            value: Expr::Ident("two")
                        },
                        FuncArg {
                            name: Some("verbatim"),
                            value: Expr::Verbatim("1 + 2")
                        },
                    ]
                }
            ))
        );
    }
}
