use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{cut, eof, recognize},
    multi::fold_many0,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

use crate::{dart::comment::Comment, parser::common::*};

pub fn comments(s: &str) -> IResult<&str, Comment> {
    alt((
        comment_single_line.map(Comment::SingleLine),
        comment_multi_line.map(Comment::MultiLine),
    ))(s)
}

fn comment_single_line(s: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("//"), is_not("\r\n"), alt((br, eof)))))(s)
}

fn comment_multi_line(s: &str) -> IResult<&str, &str> {
    recognize(preceded(
        tag("/*"),
        cut(terminated(
            recognize(fold_many0(
                alt((is_not("/**/"), comment_multi_line)),
                // alt((not(alt((tag("/*"), tag("*/")))), multi_line_comment?)),
                || {},
                |_, _| {},
            )),
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
        assert_eq!(
            comment_multi_line("/* A comment */"),
            Ok(("", "/* A comment */"))
        );
    }

    #[test]
    fn comment_multi_line_nested_test() {
        assert_eq!(
            comment_multi_line("/* A comment\n /* another comment */\n*/"),
            Ok(("", "/* A comment\n /* another comment */\n*/"))
        );
    }
}
