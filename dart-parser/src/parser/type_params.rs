use nom::{
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ContextError, ParseError},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::TypeParam;

use super::{
    common::spbr,
    identifier::{identifier, identifier_ext},
    PResult,
};

pub fn type_params<'s, E>(s: &'s str) -> PResult<Vec<TypeParam>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "type_params",
        preceded(
            pair(tag("<"), opt(spbr)),
            cut(terminated(
                separated_list1(pair(tag(","), opt(spbr)), terminated(type_param, opt(spbr))),
                tag(">"),
            )),
        ),
    )(s)
}

pub fn type_param<'s, E>(s: &'s str) -> PResult<TypeParam, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "type_param",
        pair(
            identifier,
            opt(preceded(
                tuple((spbr, tag("extends"), spbr)),
                cut(identifier_ext),
            )),
        )
        .map(|(name, extends)| TypeParam { name, extends }),
    )(s)
}
