use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, one_of},
    combinator::{cut, not, opt, recognize},
    error::{context, ContextError, ParseError},
    multi::many_m_n,
    sequence::{pair, preceded, terminated, tuple},
};

use super::{common::skip_many0, expr::block, ty::identifier, PResult};

/// Parse a single- or double-quoted single- or multiline string literal.
///
/// Escape sequences are recognized, but not decoded.
///
/// String interpolation syntax is not recognized and is consumed as a part of
/// the literal, as long as it doesn't make the parser fail due to nested string
/// literals.
///
/// Return the body of the string without the enclosing quotes.
pub fn string<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    let tdq = preceded(
        pair(tag("\"\"\""), opt(char('\n'))),
        cut(terminated(
            recognize(skip_many0(alt((
                is_not("$\\\""),
                terminated(tag("\""), not(tag("\"\""))),
                escape_seq,
                interpolation_expr,
            )))),
            tag("\"\"\""),
        )),
    );
    let tsq = preceded(
        pair(tag("'''"), opt(char('\n'))),
        cut(terminated(
            recognize(skip_many0(alt((
                is_not("$\\'"),
                terminated(tag("'"), not(tag("''"))),
                escape_seq,
                interpolation_expr,
            )))),
            tag("'''"),
        )),
    );
    let dq = preceded(
        tag("\""),
        cut(terminated(
            recognize(skip_many0(alt((
                is_not("$\\\"\r\n"),
                escape_seq,
                interpolation_expr,
            )))),
            tag("\""),
        )),
    );
    let sq = preceded(
        tag("'"),
        cut(terminated(
            recognize(skip_many0(alt((
                is_not("$\\'\r\n"),
                escape_seq,
                interpolation_expr,
            )))),
            tag("'"),
        )),
    );

    context("string", alt((tdq, tsq, dq, sq)))(s)
}

fn escape_seq<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        recognize(pair(char('\\'), one_of("nrfbtv$'\""))),
        recognize(pair(tag("\\x"), hex_digits(2, 2))),
        recognize(tuple((tag("\\u{"), hex_digits(1, 6), char('}')))),
        recognize(pair(tag("\\u"), hex_digits(4, 4))),
    ))(s)
}

fn hex_digits<'s, E>(m: usize, n: usize) -> impl FnMut(&'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(many_m_n(m, n, one_of("0123456789ABCDEFabcdef")))
}

fn interpolation_expr<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "interpolation_expr",
        recognize(preceded(tag("$"), alt((identifier, block)))),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn string_simple_test() {
        assert_eq!(
            string::<VerboseError<_>>(r#""as${df}'gh'"x"#),
            Ok(("x", r#"as${df}'gh'"#))
        );

        assert_eq!(
            string::<VerboseError<_>>(r#"'as${df}"gh"'x"#),
            Ok(("x", r#"as${df}"gh""#))
        );
    }

    #[test]
    fn string_interpolation_test() {
        assert_eq!(
            string::<VerboseError<_>>(r#""ab${f("\"")}cd"x"#),
            Ok(("x", r#"ab${f("\"")}cd"#))
        );
    }
}
