use nom::{
    bytes::complete::tag,
    combinator::opt,
    error::{context, ContextError, ParseError},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::EnumTy;

use super::{
    class::implements_clause, common::spbr, identifier::identifier, scope::block, PResult,
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
            block,
        )),
    )
    .map(|(name, implements, _)| EnumTy {
        name,
        implements: implements.unwrap_or(Vec::new()),
    })
    .parse(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn enum_test() {
        assert_eq!(
            enum_ty::<VerboseError<_>>("enum Angle { thirtyDegrees }x"),
            Ok((
                "x",
                EnumTy {
                    name: "Angle",
                    implements: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn enum_implements_test() {
        assert_eq!(
            enum_ty::<VerboseError<_>>("enum Angle implements Serializable { thirtyDegrees }x"),
            Ok((
                "x",
                EnumTy {
                    name: "Angle",
                    implements: vec![IdentifierExt::name("Serializable")]
                }
            ))
        );
    }
}
