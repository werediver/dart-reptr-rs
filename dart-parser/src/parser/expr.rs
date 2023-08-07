use nom::{
    branch::alt,
    bytes::complete::is_not,
    combinator::{eof, opt, recognize},
    error::{ContextError, ParseError},
    sequence::{preceded, terminated},
    Parser,
};

use crate::{
    dart::Expr,
    parser::{
        common::{skip_many1, spbr},
        scope::{any_scope, SCOPE_STOP_CHARS},
    },
};

use super::{identifier::identifier, string::string_simple, PResult};

pub fn expr<'s, E>(s: &'s str) -> PResult<Expr, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    const EXPR_STOP_CHARS_EXT: &str = "()[]{},;";
    debug_assert!(EXPR_STOP_CHARS_EXT.starts_with(SCOPE_STOP_CHARS));

    recognize(
        // Make sure something other than whitespace is consumed
        preceded(
            opt(spbr),
            skip_many1(alt((is_not(EXPR_STOP_CHARS_EXT), any_scope))),
        ),
    )
    .and_then(alt((
        terminated(identifier, eof).map(Expr::Ident),
        terminated(string_simple, eof).map(Expr::String),
        |s: &'s str| Ok((&s[s.len()..], Expr::Verbatim(s))),
    )))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn expr_string_test() {
        assert_eq!(
            expr::<VerboseError<_>>("'text'; "),
            Ok(("; ", Expr::String("text")))
        );
    }

    #[test]
    fn expr_test() {
        assert_eq!(
            expr::<VerboseError<_>>("f('text', (a) => null) + 1; "),
            Ok(("; ", Expr::Verbatim("f('text', (a) => null) + 1")))
        );
    }
}
