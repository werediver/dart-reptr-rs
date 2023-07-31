mod class;
mod common;
mod directive;
mod expr;
mod func;
mod scope;
mod string;
mod var;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{eof, recognize},
    multi::many0,
    sequence::{terminated, tuple},
    Parser,
};

use crate::{
    dart::*,
    parser::{class::class, common::*},
};

use self::{directive::directive, func::func, var::var};

type PResult<'s, T> = nom::IResult<&'s str, T>;

pub fn parse(s: &str) -> PResult<Vec<Dart>> {
    terminated(
        many0(alt((
            alt((spbr, comment)).map(Dart::Verbatim),
            directive.map(Dart::Directive),
            var.map(Dart::Var),
            func.map(Dart::Func),
            class.map(Dart::Class),
        ))),
        eof,
    )(s)
}

/// The single-line comment parser consumes the trailing line-break, because
/// that line-break terminates the comment rather than being "just" whitespace.
fn comment(s: &str) -> PResult<&str> {
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
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([VarModifier::Const]),
                        var_type: None,
                        name: "category",
                        initializer: Some("\"mixed bag\""),
                    }),
                    Dart::Verbatim("\n"),
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([
                            VarModifier::Late,
                            VarModifier::Final
                        ]),
                        var_type: Some(IdentifierExt::name("int")),
                        name: "crash_count",
                        initializer: None,
                    }),
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
                                        type_args: vec![IdentifierExt::name("void")],
                                        is_nullable: false,
                                    },
                                    IdentifierExt {
                                        name: "B",
                                        type_args: Vec::default(),
                                        is_nullable: true,
                                    },
                                ],
                                is_nullable: false,
                            },
                            IdentifierExt::name("C")
                        ],
                        body: "{\n  String name;\n}",
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Func(Func {
                        modifiers: FuncModifierSet::default(),
                        return_type: IdentifierExt {
                            name: "Map",
                            type_args: vec![
                                IdentifierExt::name("String"),
                                IdentifierExt {
                                    name: "Object",
                                    type_args: Vec::new(),
                                    is_nullable: true,
                                }
                            ],
                            is_nullable: false,
                        },
                        name: "_recordToJson",
                        params: FuncParams {
                            positional: vec![
                                FuncParam {
                                    is_required: true,
                                    modifiers: FuncParamModifierSet::default(),
                                    param_type: Some(IdentifierExt::name("Record")),
                                    name: "o",
                                    initializer: None,
                                },
                                FuncParam {
                                    is_required: false,
                                    modifiers: FuncParamModifierSet::default(),
                                    param_type: Some(IdentifierExt::name("bool")),
                                    name: "quack",
                                    initializer: Some("false")
                                }
                            ],
                            named: Vec::new(),
                        },
                        body: Some(FuncBody {
                            modifiers: FuncBodyModifierSet::default(),
                            content: FuncBodyContent::Block("{\n    print(\"Hello?\");\n}")
                        })
                    }),
                    Dart::Verbatim("\n"),
                ]
            ))
        );
    }

    const DART_MIXED: &str = r#"
import 'dart:math';
import 'package:path/path.dart' as p;

part 'types.g.dart';

// A comment

const category = "mixed bag";
late final int crash_count;

class Base {
  String id;
}

class Record extends Base implements A<Future<void>, B?>, C {
  String name;
}

Map<String, Object?> _recordToJson(Record o, [bool quack = false]) {
    print("Hello?");
}
"#;
}
