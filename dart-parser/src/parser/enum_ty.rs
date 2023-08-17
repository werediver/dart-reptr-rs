use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success},
    error::{context, ContextError, ParseError},
    multi::{many0, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    class::ClassMember,
    enum_ty::{EnumMember, EnumValue},
    EnumTy,
};

use super::{
    annotation::annotation,
    class::{class_member, implements_clause},
    comment::comment,
    common::spbr,
    func_call::func_args,
    identifier::identifier,
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

fn enum_body<'s, E>(s: &'s str) -> PResult<(Vec<EnumMember>, Vec<ClassMember>), E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "enum_body",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                pair(
                    terminated(
                        separated_list0(
                            pair(tag(","), opt(spbr)),
                            terminated(enum_value_ext, opt(spbr)),
                        ),
                        opt(pair(tag(","), opt(spbr))),
                    )
                    .map(|items| items.into_iter().flatten().collect::<Vec<_>>()),
                    alt((
                        preceded(
                            pair(tag(";"), opt(spbr)),
                            many0(terminated(class_member, opt(spbr))),
                        ),
                        success(()).map(|_| Vec::new()),
                    )),
                ),
                tag("}"),
            )),
        ),
    )(s)
}

fn enum_value_ext<'s, E>(s: &'s str) -> PResult<Vec<EnumMember>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "enum_value_ext",
        pair(
            many0(alt((
                terminated(comment, opt(spbr)).map(EnumMember::Comment),
                terminated(annotation, opt(spbr)).map(EnumMember::Annotation),
            ))),
            enum_value,
        )
        .map(|(meta, value)| {
            let mut value_ext = meta;
            value_ext.push(EnumMember::Value(value));

            value_ext
        }),
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

    use crate::dart::{Annotation, Comment, FuncCall, IdentifierExt};

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
                    values: vec![EnumMember::Value(EnumValue {
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
                    values: vec![
                        EnumMember::Comment(Comment::SingleLine("// Here it comes. Big...\n")),
                        EnumMember::Annotation(Annotation::FuncCall(FuncCall {
                            ident: IdentifierExt::name("Badaboom"),
                            args: Vec::new()
                        })),
                        EnumMember::Value(EnumValue {
                            name: "thirtyDegrees",
                            args: Vec::new(),
                        })
                    ],
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
                    implements: vec![IdentifierExt::name("Serializable")],
                    values: vec![EnumMember::Value(EnumValue {
                        name: "thirtyDegrees",
                        args: Vec::new(),
                    })],
                    members: Vec::new(),
                }
            ))
        );
    }
}
