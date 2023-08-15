use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::{cut, fail, opt, recognize},
    error::{context, ContextError, ParseError},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::IdentifierExt;

use super::{common::spbr, PResult};

pub fn identifier<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    // Based on [Keywords](https://dart.dev/language/keywords).
    const RESERVED: [&str; 26] = [
        "assert", "break", "case", "catch", "class", "const", "continue", "default", "do", "else",
        "enum", "extends", "finally", "for", "if", "in", "is", "new", "rethrow", "return",
        "switch", "throw", "try", "var", "while",
        "with",
        // It is desirable to recognize the following words as identifiers
        // "false", "null", "super", "this", "true", "void",
    ];

    fn is_start_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }

    fn is_part_char(c: char) -> bool {
        is_start_char(c) || c.is_ascii_digit()
        // Parse composite identifiers as a single identifier
        || c == '.'
    }

    context("identifier", |s| {
        let (tail, id) = recognize(pair(
            take_while_m_n(1, 1, is_start_char),
            take_while(is_part_char),
        ))(s)?;

        if RESERVED.contains(&id) {
            fail(s)
        } else {
            Ok((tail, id))
        }
    })(s)
}

/// Parse an identifier with type arguments and the nullability indicator (e.g. `x`, `Future<int>?`).
pub fn identifier_ext<'s, E>(s: &'s str) -> PResult<IdentifierExt, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "identifier_ext",
        tuple((
            identifier,
            opt(preceded(opt(spbr), type_args)),
            opt(preceded(opt(spbr), tag("?"))),
        ))
        .map(|(name, args, nullability_ind)| IdentifierExt {
            name,
            type_args: args.unwrap_or(Vec::default()),
            is_nullable: nullability_ind.is_some(),
        }),
    )
    .parse(s)
}

fn type_args<'s, E>(s: &'s str) -> PResult<Vec<IdentifierExt>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "type_args",
        preceded(
            pair(tag("<"), opt(spbr)),
            cut(terminated(
                separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), identifier_ext),
                pair(opt(spbr), tag(">")),
            )),
        ),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn identifier_ext_test() {
        assert_eq!(
            identifier_ext::<VerboseError<_>>("Map<String, Object>? "),
            Ok((
                " ",
                IdentifierExt {
                    name: "Map",
                    type_args: vec![IdentifierExt::name("String"), IdentifierExt::name("Object"),],
                    is_nullable: true,
                }
            ))
        );
    }

    #[test]
    fn identifier_ext_nested_test() {
        assert_eq!(
            identifier_ext::<VerboseError<_>>("Map<String, List<int>> "),
            Ok((
                " ",
                IdentifierExt {
                    name: "Map",
                    type_args: vec![
                        IdentifierExt::name("String"),
                        IdentifierExt {
                            name: "List",
                            type_args: vec![IdentifierExt::name("int")],
                            is_nullable: false,
                        }
                    ],
                    is_nullable: false,
                }
            ))
        );
    }
}
