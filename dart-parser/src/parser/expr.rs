use nom::{
    branch::alt,
    bytes::complete::is_not,
    combinator::recognize,
    error::{ContextError, ParseError},
};

use crate::parser::{
    common::skip_many0,
    scope::{any_scope, SCOPE_STOP_CHARS},
};

use super::PResult;

/// This is a pretty shady expression parser.
pub fn expr<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    const EXPR_STOP_CHARS_EXT: &str = "()[]{},;";
    debug_assert!(EXPR_STOP_CHARS_EXT.starts_with(SCOPE_STOP_CHARS));

    recognize(skip_many0(alt((is_not(EXPR_STOP_CHARS_EXT), any_scope))))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn expr_string_test() {
        assert_eq!(
            expr::<VerboseError<_>>("\"text\"; "),
            Ok(("; ", "\"text\""))
        );
    }

    #[test]
    fn expr_test() {
        assert_eq!(
            expr::<VerboseError<_>>("f(\"text\", (a) => null) + 1; "),
            Ok(("; ", "f(\"text\", (a) => null) + 1"))
        );
    }
}
