use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{cut, recognize},
    error::{context, ContextError, ParseError},
    multi::many0_count,
    sequence::{preceded, terminated},
};

use super::PResult;

/// Parse a single- or double-quoted string literal without escape-sequences.
///
/// Any backslash in the literal makes the parser fail.
///
/// String interpolation syntax is not recognized and is consumed as a part of
/// the literal, as long as it doesn't make the parser fail due to nested string
/// literals.
///
/// Return the body of the string without the enclosing quotes.
pub fn string_simple<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    let dq = preceded(
        tag("\""),
        cut(terminated(
            recognize(many0_count(is_not("\\\"\r\n"))),
            tag("\""),
        )),
    );
    let sq = preceded(
        tag("'"),
        cut(terminated(
            recognize(many0_count(is_not("\\'\r\n"))),
            tag("'"),
        )),
    );

    context("string_simple", alt((dq, sq)))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn string_simple_test() {
        assert_eq!(
            string_simple::<VerboseError<_>>(r#""as${df}'gh'"x"#),
            Ok(("x", r#"as${df}'gh'"#))
        );

        assert_eq!(
            string_simple::<VerboseError<_>>(r#"'as${df}"gh"'x"#),
            Ok(("x", r#"as${df}"gh""#))
        );
    }
}
