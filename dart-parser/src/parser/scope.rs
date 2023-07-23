use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::char,
    combinator::{cut, recognize},
    sequence::{preceded, terminated},
};

use crate::parser::common::skip_many0;

use super::PResult;

pub const SCOPE_STOP_CHARS: &str = "()[]{}";

pub fn any_scope(s: &str) -> PResult<&str> {
    recognize(alt((scope('(', ')'), scope('[', ']'), scope('{', '}'))))(s)
}

pub fn block(s: &str) -> PResult<&str> {
    recognize(scope('{', '}'))(s)
}

/// Note that this parser strips the outermost brackets.
fn scope<'s>(open: char, close: char) -> impl FnMut(&'s str) -> PResult<&'s str> {
    preceded(
        char(open),
        cut(terminated(
            recognize(skip_many0(alt((is_not(SCOPE_STOP_CHARS), any_scope)))),
            char(close),
        )),
    )
}
