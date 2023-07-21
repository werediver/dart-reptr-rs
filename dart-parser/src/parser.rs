mod class;
mod common;
mod directive;
mod member;
mod string;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{eof, recognize},
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    dart::*,
    parser::{class::class, common::*},
};

use self::directive::directive;

pub fn parse(s: &str) -> IResult<&str, Vec<Dart>> {
    let (s, items) = many0(alt((
        alt((spbr, comment)).map(Dart::Verbatim),
        directive.map(Dart::Directive),
        class.map(Dart::Class),
    )))(s)?;
    let (s, _) = eof(s)?;

    Ok((s, items))
}

fn comment(s: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("//"), is_not("\r\n"), alt((br, eof)))))(s)
}

#[cfg(test)]
mod tests {
    use crate::dart::directive::{Directive, Import};

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
    fn mixed_test() {
        assert_eq!(
            parse(DART_MIXED.trim_start()),
            Ok((
                "",
                vec![
                    Dart::Directive(Directive::Import(Import::target("dart:math"))),
                    Dart::Verbatim("\n"),
                    Dart::Directive(Directive::Import(Import::target_as(
                        "package:path/path.dart",
                        "p"
                    ))),
                    Dart::Verbatim("\n\n"),
                    Dart::Directive(Directive::Part("types.g.dart")),
                    Dart::Verbatim("\n\n"),
                    Dart::Verbatim("// A comment\n"),
                    Dart::Verbatim("\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Base",
                        extends: None,
                        implements: Vec::default(),
                        body: "{\n  String id;\n}",
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Record",
                        extends: Some(IdentifierExt::name("Base")),
                        implements: vec![
                            IdentifierExt {
                                name: "A",
                                type_args: vec![
                                    IdentifierExt {
                                        name: "Future",
                                        type_args: vec![IdentifierExt::name("void")]
                                    },
                                    IdentifierExt::name("B")
                                ]
                            },
                            IdentifierExt::name("C")
                        ],
                        body: "{\n  String name;\n}",
                    }),
                    Dart::Verbatim("\n")
                ]
            ))
        );
    }

    const DART_MIXED: &str = r#"
import 'dart:math';
import 'package:path/path.dart' as p;

part 'types.g.dart';

// A comment

class Base {
  String id;
}

class Record extends Base implements A<Future<void>, B>, C {
  String name;
}
"#;
}
