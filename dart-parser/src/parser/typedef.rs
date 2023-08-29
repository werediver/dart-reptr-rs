use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, success},
    error::{context, ContextError, ParseError},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::TypeDef;

use super::{
    common::spbr,
    ty::{identifier, ty},
    type_params::type_params,
    PResult,
};

pub fn typedef<'s, E>(s: &'s str) -> PResult<TypeDef, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "typedef",
        preceded(
            terminated(tag("typedef"), spbr),
            tuple((
                terminated(identifier, opt(spbr)),
                terminated(
                    alt((type_params, success(()).map(|_| Vec::new()))),
                    tuple((opt(spbr), tag("="), opt(spbr))),
                ),
                terminated(ty, pair(opt(spbr), tag(";"))),
            )),
        )
        .map(|(name, type_params, target)| TypeDef {
            name,
            type_params,
            target,
        }),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{ty::Type, NotFuncType};

    use super::*;

    #[test]
    fn typedef_test() {
        assert_eq!(
            typedef::<VerboseError<_>>("typedef StringList = List<String>;x"),
            Ok((
                "x",
                TypeDef {
                    name: "StringList",
                    type_params: Vec::new(),
                    target: Type::NotFunc(NotFuncType {
                        name: "List",
                        type_args: vec![Type::NotFunc(NotFuncType::name("String"))],
                        is_nullable: false
                    })
                }
            ))
        );
    }
}
