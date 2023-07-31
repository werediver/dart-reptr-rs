use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{cut, recognize},
    error::{context, ContextError, ParseError},
    sequence::{preceded, terminated},
};

use crate::parser::common::skip_many0;

use super::PResult;

pub const SCOPE_STOP_CHARS: &str = "()[]{}";

pub fn any_scope<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(alt((scope('(', ')'), scope('[', ']'), scope('{', '}'))))(s)
}

pub fn block<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(scope('{', '}'))(s)
}

/// Note that this parser strips the outermost brackets.
fn scope<'s, E>(open: char, close: char) -> impl FnMut(&'s str) -> PResult<&'s str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "scope",
        preceded(
            char(open),
            cut(terminated(
                recognize(skip_many0(alt((is_not(SCOPE_STOP_CHARS), any_scope)))),
                char(close),
            )),
        ),
    )
}
