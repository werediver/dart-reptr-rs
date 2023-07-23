use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while, take_while_m_n},
    character::complete::char,
    combinator::{cut, opt, recognize},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::IdentifierExt;

use super::PResult;

/// Parse one or more whitespace characters, excluding line breaks.
pub fn sp(s: &str) -> PResult<&str> {
    is_a(" \t")(s)
}

/// Parse one or more whitespace characters, including line breaks.
pub fn spbr(s: &str) -> PResult<&str> {
    is_a(" \t\r\n")(s)
}

/// Parse exactly one line break.
pub fn br(s: &str) -> PResult<&str> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(s)
}

pub fn identifier(s: &str) -> PResult<&str> {
    fn is_start_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }

    fn is_part_char(c: char) -> bool {
        is_start_char(c) || c.is_ascii_digit()
    }

    recognize(pair(
        take_while_m_n(1, 1, is_start_char),
        take_while(is_part_char),
    ))(s)
}

/// Parse an identifier with type arguments and the nullability indicator (e.g. `Future<int>?`).
pub fn identifier_ext(s: &str) -> PResult<IdentifierExt> {
    tuple((
        identifier,
        opt(preceded(
            tuple((opt(spbr), tag("<"), opt(spbr))),
            cut(terminated(
                separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), identifier_ext),
                pair(opt(spbr), tag(">")),
            )),
        )),
        opt(preceded(opt(spbr), tag("?"))),
    ))
    .map(|(name, args, nullability_ind)| IdentifierExt {
        name,
        type_args: args.unwrap_or(Vec::default()),
        is_nullable: nullability_ind.is_some(),
    })
    .parse(s)
}

pub fn block(s: &str) -> PResult<&str> {
    recognize(preceded(
        char('{'),
        cut(terminated(
            recognize(fold_many0(alt((is_not("{}"), block)), || {}, |_, _| {})),
            char('}'),
        )),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sp_test() {
        let s = "  \n\t\r\nx";
        assert_eq!(spbr(s), Ok(("x", "  \n\t\r\n")));
    }

    #[test]
    fn identifier_ext_test() {
        assert_eq!(
            identifier_ext("Map<String, Object>? "),
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
            identifier_ext("Map<String, List<int>> "),
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
