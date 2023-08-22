use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::cut,
    error::{ContextError, ParseError},
    sequence::preceded,
    Parser,
};

use crate::dart::Annotation;

use super::{func_call::func_call, ty::identifier, PResult};

pub fn annotation<'s, E>(s: &'s str) -> PResult<Annotation, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        tag("@"),
        cut(alt((
            func_call.map(Annotation::FuncCall),
            identifier.map(Annotation::Ident),
        ))),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{func_call::FuncArg, Expr, FuncCall, NotFuncType};

    use super::*;

    #[test]
    fn annotation_const_test() {
        assert_eq!(
            annotation::<VerboseError<_>>("@immutable\nx"),
            Ok(("\nx", Annotation::Ident("immutable")))
        );
    }

    #[test]
    fn annotation_constructor_test() {
        assert_eq!(
            annotation::<VerboseError<_>>("@JsonSerializable()\nx"),
            Ok((
                "\nx",
                Annotation::FuncCall(FuncCall {
                    ident: NotFuncType::name("JsonSerializable"),
                    args: Vec::new(),
                })
            ))
        );
    }

    #[test]
    fn annotation_constructor_args_test() {
        assert_eq!(
            annotation::<VerboseError<_>>(
                "@JsonSerializable(1, 2.0, named: 'three', expr: 1 + 2)\nx"
            ),
            Ok((
                "\nx",
                Annotation::FuncCall(FuncCall {
                    ident: NotFuncType::name("JsonSerializable"),
                    args: vec![
                        FuncArg {
                            name: None,
                            value: Expr::Verbatim("1"),
                        },
                        FuncArg {
                            name: None,
                            value: Expr::Verbatim("2.0"),
                        },
                        FuncArg {
                            name: Some("named"),
                            value: Expr::String("three"),
                        },
                        FuncArg {
                            name: Some("expr"),
                            value: Expr::Verbatim("1 + 2"),
                        }
                    ],
                }),
            ))
        );
    }
}
