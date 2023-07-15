mod class;
mod common;
mod string;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{cut, eof, map, opt, recognize},
    multi::many0,
    sequence::{pair, preceded, tuple},
    IResult,
};

use crate::{
    dart::*,
    parser::{class::class, common::*},
};

use self::string::string_simple;

pub fn parse(s: &str) -> IResult<&str, Vec<Dart>> {
    let (s, items) = many0(alt((
        map(alt((spbr, comment)), Dart::Verbatim),
        map(import, Dart::Import),
        map(class, Dart::Class),
    )))(s)?;
    let (s, _) = eof(s)?;

    Ok((s, items))
}

fn comment(s: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("//"), is_not("\r\n"), alt((br, eof)))))(s)
}

fn import(s: &str) -> IResult<&str, Import> {
    preceded(
        pair(tag("import"), spbr),
        cut(map(
            tuple((string_simple, opt(spbr), tag(";"))),
            |(s, _, _)| Import { target: s },
        )),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_test() {
        assert_eq!(comment("// A comment\nx"), Ok(("x", "// A comment\n")));
    }

    #[test]
    fn comment_eof_test() {
        assert_eq!(comment("// A comment"), Ok(("", "// A comment")));
    }

    #[test]
    fn import_test() {
        assert_eq!(
            import("import 'dart:math';x"),
            Ok((
                "x",
                Import {
                    target: "dart:math"
                }
            ))
        );
    }

    #[test]
    fn mixed_test() {
        assert_eq!(
            parse(DART_BASIC.trim_start()),
            Ok((
                "",
                vec![
                    Dart::Import(Import {
                        target: "dart:math"
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Verbatim("// A comment\n"),
                    Dart::Verbatim("\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Base",
                        extends: None,
                        body: "{\n  String id;\n}",
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Record",
                        extends: Some("Base"),
                        body: "{\n  String name;\n}",
                    }),
                    Dart::Verbatim("\n")
                ]
            ))
        );
    }

    const DART_BASIC: &str = r#"
import 'dart:math';

// A comment

class Base {
  String id;
}

class Record extends Base {
  String name;
}
"#;
}
