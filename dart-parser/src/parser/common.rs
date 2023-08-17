use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    error::ParseError,
    multi::{fold_many0, fold_many1},
    InputLength, Parser,
};

use super::PResult;

/// Convert irrecoverable errors from into recoverable.
///
/// For use in shady parsers.
pub fn uncut<P, I, O, E>(mut p: P) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    move |s| match p.parse(s) {
        result @ Ok(_) => result,
        result @ Err(nom::Err::Incomplete(_)) => result,
        result @ Err(nom::Err::Error(_)) => result,
        // Convert an irrecoverable error into a recoverable
        Err(nom::Err::Failure(e)) => Err(nom::Err::Error(e)),
    }
}

pub fn skip_many0<P, I, O, E>(p: P) -> impl FnMut(I) -> nom::IResult<I, (), E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    fold_many0(p, || {}, |_, _| {})
}

pub fn skip_many1<P, I, O, E>(p: P) -> impl FnMut(I) -> nom::IResult<I, (), E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    fold_many1(p, || {}, |_, _| {})
}

/// Parse one or more whitespace characters, including line breaks.
pub fn spbr<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<&str, E> {
    is_a(" \t\r\n")(s)
}

/// Parse exactly one line break.
pub fn br<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<&str, E> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn sp_test() {
        let s = "  \n\t\r\nx";
        assert_eq!(spbr::<VerboseError<_>>(s), Ok(("x", "  \n\t\r\n")));
    }
}
