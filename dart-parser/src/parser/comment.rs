use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{cut, eof, not, recognize},
    error::{context, ContextError, ParseError},
    sequence::{preceded, terminated},
    Parser,
};

use crate::{dart::comment::Comment, parser::common::*};

use super::PResult;

/// The single-line comment parser consumes the trailing line-break, because
/// that line-break terminates the comment rather than being "just" whitespace.
pub fn comment<'s, E>(s: &'s str) -> PResult<Comment, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        comment_single_line.map(Comment::SingleLine),
        comment_multi_line.map(Comment::MultiLine),
    ))(s)
}

fn comment_single_line<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "comment_single_line",
        recognize(preceded(
            tag("//"),
            cut(terminated(is_not("\r\n"), alt((br, eof)))),
        )),
    )(s)
}

fn comment_multi_line<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "comment_multi_line",
        recognize(preceded(
            tag("/*"),
            cut(terminated(
                recognize(skip_many0(alt((
                    is_not("*/"),
                    terminated(tag("*"), not(char('/'))),
                    terminated(tag("/"), not(char('*'))),
                    comment_multi_line,
                )))),
                tag("*/"),
            )),
        )),
    )(s)
}

#[cfg(test)]
mod tests {

    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn comment_single_line_test() {
        assert_eq!(
            comment_single_line::<VerboseError<_>>("// A comment\nx"),
            Ok(("x", "// A comment\n"))
        );
    }

    #[test]
    fn comment_single_line_eof_test() {
        assert_eq!(
            comment_single_line::<VerboseError<_>>("// A comment"),
            Ok(("", "// A comment"))
        );
    }

    #[test]
    fn comment_multi_line_test() {
        assert_eq!(
            comment_multi_line::<VerboseError<_>>("/* / */"),
            Ok(("", "/* / */"))
        );
    }

    #[test]
    fn comment_multi_line_nested_test() {
        assert_eq!(
            comment_multi_line::<VerboseError<_>>("/* A comment\n /* another comment \n */\n*/"),
            Ok(("", "/* A comment\n /* another comment \n */\n*/"))
        );
    }
}
