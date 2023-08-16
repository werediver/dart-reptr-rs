use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success},
    error::{context, ContextError, ParseError},
    multi::{many0, separated_list0},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{class::ClassMember, enum_ty::EnumValue, EnumTy};

use super::{
    class::{class_member, implements_clause},
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

fn enum_body<'s, E>(s: &'s str) -> PResult<(Vec<EnumValue>, Vec<ClassMember>), E>
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
                            terminated(enum_value, opt(spbr)),
                        ),
                        opt(pair(tag(","), opt(spbr))),
                    ),
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

    use crate::dart::{func_like::FuncParams, IdentifierExt};

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
                    values: vec![EnumValue {
                        name: "thirtyDegrees",
                        args: Vec::new(),
                    }],
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
                    values: vec![EnumValue {
                        name: "thirtyDegrees",
                        args: Vec::new(),
                    }],
                    members: Vec::new(),
                }
            ))
        );
    }
}
