use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{cut, eof, not, recognize},
    sequence::{preceded, terminated, tuple},
    Parser,
};

use crate::{dart::comment::Comment, parser::common::*};

use super::PResult;

pub fn comment(s: &str) -> PResult<Comment> {
    alt((
        comment_single_line.map(Comment::SingleLine),
        comment_multi_line.map(Comment::MultiLine),
    ))(s)
}

fn comment_single_line(s: &str) -> PResult<&str> {
    recognize(tuple((tag("//"), is_not("\r\n"), alt((br, eof)))))(s)
}

fn comment_multi_line(s: &str) -> PResult<&str> {
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
    ))(s)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn comment_single_line_test() {
        assert_eq!(
            comment_single_line("// A comment\nx"),
            Ok(("x", "// A comment\n"))
        );
    }

    #[test]
    fn comment_single_line_eof_test() {
        assert_eq!(
            comment_single_line("// A comment"),
            Ok(("", "// A comment"))
        );
    }

    #[test]
    fn comment_multi_line_test() {
        assert_eq!(comment_multi_line("/* / */"), Ok(("", "/* / */")));
    }

    #[test]
    fn comment_multi_line_nested_test() {
        assert_eq!(
            comment_multi_line("/* A comment\n /* another comment \n */\n*/"),
            Ok(("", "/* A comment\n /* another comment \n */\n*/"))
        );
    }
}
