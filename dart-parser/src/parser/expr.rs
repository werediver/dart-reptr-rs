use nom::{
    branch::alt,
    combinator::{eof, opt, recognize},
    error::{ContextError, ParseError},
    sequence::{preceded, terminated},
    Parser,
};

use crate::{dart::Expr, parser::common::spbr};

use super::{comment, common::uncut, string::string, ty::identifier, PResult};

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{cut, success},
    error::context,
};

use super::{common::skip_many1, ty::type_args};

pub fn expr<'s, E>(s: &'s str) -> PResult<Expr, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "expr",
        recognize(
            // Make sure something other than whitespace is consumed
            preceded(opt(spbr), expr_body),
        )
        .and_then(alt((
            terminated(identifier, eof).map(Expr::Ident),
            terminated(string, eof).map(Expr::String),
            |s: &'s str| Ok((&s[s.len()..], Expr::Verbatim(s))),
        ))),
    )
    .parse(s)
}

pub fn block<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(scope('{', '}'))(s)
}

fn any_scope<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(alt((scope('(', ')'), scope('[', ']'), scope('{', '}'))))(s)
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
            cut(terminated(alt((scope_body, success(""))), char(close))),
        ),
    )
}

const SCOPE_STOP_CHARS: &str = "<>()[]{}=/r'\"";

fn scope_body<'s, E>(s: &'s str) -> PResult<&'s str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    recognize(skip_many1(alt((is_not(SCOPE_STOP_CHARS), body_item))))(s)
}

fn expr_body<'s, E>(s: &'s str) -> PResult<&'s str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    const EXPR_STOP_CHARS: &str = "<>()[]{}=/r'\",;";
    debug_assert!(EXPR_STOP_CHARS.starts_with(SCOPE_STOP_CHARS));

    recognize(skip_many1(alt((is_not(EXPR_STOP_CHARS), body_item))))(s)
}

fn body_item<'s, E>(s: &'s str) -> PResult<&'s str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        tag("=>"), // Consume '>', if it's a part of '=>'
        tag("="),
        uncut(recognize(type_args)),
        tag("<"), // LT/LTE
        tag(">"), // GT/GTE
        recognize(comment),
        tag("/"),
        string,
        tag("r"),
        any_scope,
    ))(s)
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

    #[test]
    fn expr_typed_list_test() {
        assert_eq!(
            expr::<VerboseError<_>>("<String>['asdf', 'jkl;']; "),
            Ok(("; ", Expr::Verbatim("<String>['asdf', 'jkl;']")))
        );
    }

    #[test]
    fn expr_list_test() {
        assert_eq!(
            expr::<VerboseError<_>>("[\n'asdf',\n'jkl;'\n]; "),
            Ok(("; ", Expr::Verbatim("[\n'asdf',\n'jkl;'\n]")))
        );
    }

    #[test]
    fn expr_set_test() {
        assert_eq!(
            expr::<VerboseError<_>>("<String>{'asdf', 'jkl;'}; "),
            Ok(("; ", Expr::Verbatim("<String>{'asdf', 'jkl;'}")))
        );
    }

    #[test]
    fn expr_map_test() {
        assert_eq!(
            expr::<VerboseError<_>>("<String, Object?>{'asdf': 123, 'jkl;': 456}; "),
            Ok((
                "; ",
                Expr::Verbatim("<String, Object?>{'asdf': 123, 'jkl;': 456}")
            ))
        );
    }
}
