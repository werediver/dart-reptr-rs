mod class;
mod comment;
mod common;
mod directive;
mod string;

use std::str;

use nom::{branch::alt, combinator::eof, multi::many0, IResult, Parser};

use crate::{dart::*, parser::class::class};

use self::{comment::comments, common::spbr, directive::directive};

pub fn parse(s: &str) -> IResult<&str, Vec<Dart>> {
    let (s, items) = many0(alt((
        directive.map(Dart::Directive),
        spbr.map(Dart::Verbatim),
        comments.map(Dart::Comment),
        class.map(Dart::Class),
    )))(s)?;
    let (s, _) = eof(s)?;

    Ok((s, items))
}

#[cfg(test)]
mod tests {

    use crate::dart::{
        comment::Comment,
        directive::{Directive, Import},
    };

    use super::*;

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
                    Dart::Comment(Comment::SingleLine("// A comment\n")),
                    Dart::Comment(Comment::MultiLine("/*\nAnother comment\n*/")),
                    Dart::Verbatim("\n\n"),
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
/*
Another comment
*/

class Base {
  String id;
}

class Record extends Base implements A<Future<void>, B>, C {
  String name;
}
"#;
}
