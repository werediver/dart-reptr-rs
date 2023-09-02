use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success},
    error::{context, ContextError, ParseError},
    multi::many0,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{class::ClassMember, enum_ty::EnumValue, EnumTy, WithMeta};

use super::{
    class::{class_member, implements_clause},
    common::{sep_list, spbr, spbrc, SepMode},
    func_call::func_args,
    meta::with_meta,
    ty::identifier,
    PResult,
};

pub fn enum_ty<'s, E>(s: &'s str) -> PResult<EnumTy, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "enum_ty",
        tuple((
            terminated(preceded(pair(tag("enum"), spbr), identifier), opt(spbr)),
            opt(terminated(implements_clause, opt(spbr))),
            enum_body,
        )),
    )
    .map(|(name, implements, (values, members))| EnumTy {
        name,
        implements: implements.unwrap_or(Vec::new()),
        values,
        members,
    })
    .parse(s)
}

#[allow(clippy::type_complexity)]
fn enum_body<'s, E>(
    s: &'s str,
) -> PResult<(Vec<WithMeta<EnumValue>>, Vec<WithMeta<ClassMember>>), E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "enum_body",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                pair(
                    sep_list(
                        0,
                        SepMode::AllowTrailing,
                        pair(tag(","), opt(spbr)),
                        terminated(with_meta(enum_value), opt(spbrc)),
                    ),
                    alt((
                        preceded(
                            pair(tag(";"), opt(spbr)),
                            many0(terminated(with_meta(class_member), opt(spbr))),
                        ),
                        success(()).map(|_| Vec::new()),
                    )),
                ),
                tag("}"),
            )),
        ),
    )(s)
}

fn enum_value<'s, E>(s: &'s str) -> PResult<EnumValue, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "enum_value",
        tuple((
            terminated(identifier, opt(spbr)),
            alt((
                terminated(func_args, opt(spbr)),
                success(()).map(|_| Vec::new()),
            )),
        ))
        .map(|(name, params)| EnumValue { name, args: params }),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{meta::Meta, Annotation, Comment, FuncCall, NotFuncType};

    use super::*;

    #[test]
    fn enum_test() {
        assert_eq!(
            enum_ty::<VerboseError<_>>("enum AnyAngle { thirtyDegrees }x"),
            Ok((
                "x",
                EnumTy {
                    name: "AnyAngle",
                    implements: Vec::new(),
                    values: vec![WithMeta::value(EnumValue {
                        name: "thirtyDegrees",
                        args: Vec::new(),
                    })],
                    members: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn enum_ext_test() {
        assert_eq!(
            enum_ty::<VerboseError<_>>(
                "enum AnyAngle {\n  // Here it comes. Big...\n  @Badaboom()\n  thirtyDegrees\n}x"
            ),
            Ok((
                "x",
                EnumTy {
                    name: "AnyAngle",
                    implements: Vec::new(),
                    values: vec![WithMeta::new(
                        vec![
                            Meta::Comment(Comment::SingleLine("// Here it comes. Big...\n")),
                            Meta::Annotation(Annotation::FuncCall(FuncCall {
                                ident: NotFuncType::name("Badaboom"),
                                args: Vec::new()
                            }))
                        ],
                        EnumValue {
                            name: "thirtyDegrees",
                            args: Vec::new(),
                        }
                    )],
                    members: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn enum_implements_test() {
        assert_eq!(
            enum_ty::<VerboseError<_>>("enum AnyAngle implements Serializable { thirtyDegrees }x"),
            Ok((
                "x",
                EnumTy {
                    name: "AnyAngle",
                    implements: vec![NotFuncType::name("Serializable")],
                    values: vec![WithMeta::value(EnumValue {
                        name: "thirtyDegrees",
                        args: Vec::new(),
                    })],
                    members: Vec::new(),
                }
            ))
        );
    }
}
