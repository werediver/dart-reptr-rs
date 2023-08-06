mod class;
mod comment;
mod common;
mod directive;
mod enum_ty;
mod expr;
mod func;
mod scope;
mod string;
mod var;

use std::str;

use nom::{
    branch::alt,
    combinator::{eof, opt},
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{preceded, terminated},
    Parser,
};

use crate::{dart::*, parser::class::class};

use self::{
    comment::comment, common::spbr, directive::directive, enum_ty::enum_ty, func::func, var::var,
};

type PResult<'s, T, E> = Result<(&'s str, T), nom::Err<E>>;

pub fn parse<'s, E>(s: &'s str) -> PResult<Vec<Dart>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        opt(spbr),
        terminated(
            many0(terminated(
                alt((
                    comment.map(Dart::Comment),
                    directive.map(Dart::Directive),
                    var.map(Dart::Var),
                    func.map(Dart::Func),
                    class.map(Dart::Class),
                    enum_ty.map(Dart::Enum),
                )),
                opt(spbr),
            )),
            eof,
        ),
    )(s)
}

#[cfg(test)]
mod tests {

    use nom::error::VerboseError;

    use crate::dart::{
        class::{ClassMember, ClassModifier, ClassModifierSet, Constructor},
        comment::Comment,
        directive::{Directive, Import},
        func::{
            FuncBody, FuncBodyContent, FuncModifierSet, FuncParam, FuncParamModifierSet, FuncParams,
        },
        var::{VarModifier, VarModifierSet},
    };

    use super::*;

    #[test]
    fn mixed_test() {
        assert_eq!(
            parse::<VerboseError<_>>(DART_MIXED.trim_start()),
            Ok((
                "",
                vec![
                    Dart::Directive(Directive::Import(Import::target("dart:math"))),
                    Dart::Directive(Directive::Import(Import::target_as(
                        "package:path/path.dart",
                        "p"
                    ))),
                    Dart::Directive(Directive::Part("types.g.dart")),
                    Dart::Comment(Comment::SingleLine("// A comment\n")),
                    Dart::Comment(Comment::MultiLine("/*\nAnother comment\n*/")),
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([VarModifier::Const]),
                        var_type: None,
                        name: "category",
                        initializer: Some("\"mixed bag\""),
                    }),
                    Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([
                            VarModifier::Late,
                            VarModifier::Final
                        ]),
                        var_type: Some(IdentifierExt::name("int")),
                        name: "crash_count",
                        initializer: None,
                    }),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Base",
                        extends: None,
                        implements: Vec::default(),
                        body: vec![
                            ClassMember::Verbatim("\n  "),
                            ClassMember::Constructor(Constructor {
                                modifier: None,
                                name: "Base",
                                params: FuncParams {
                                    positional: vec![FuncParam {
                                        is_required: true,
                                        modifiers: FuncParamModifierSet::default(),
                                        param_type: None,
                                        name: "this.id",
                                        initializer: None
                                    }],
                                    named: Vec::new(),
                                },
                                body: None,
                            }),
                            ClassMember::Verbatim("\n\n  "),
                            ClassMember::Var(Var {
                                modifiers: VarModifierSet::from_iter([VarModifier::Final]),
                                var_type: Some(IdentifierExt::name("String")),
                                name: "id",
                                initializer: None,
                            }),
                            ClassMember::Verbatim("\n"),
                        ],
                    }),
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
                        body: vec![
                            ClassMember::Verbatim("\n  "),
                            ClassMember::Var(Var {
                                modifiers: VarModifierSet::default(),
                                var_type: Some(IdentifierExt::name("String")),
                                name: "name",
                                initializer: None,
                            }),
                            ClassMember::Verbatim("\n"),
                        ],
                    }),
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
                            modifier: None,
                            content: FuncBodyContent::Block("{\n    print(\"Hello?\");\n}")
                        })
                    }),
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
const category = "mixed bag";
late final int crash_count;

class Base {
  Base(this.id);

  final String id;
}

class Record extends Base implements A<Future<void>, B?>, C {
  String name;
}

Map<String, Object?> _recordToJson(Record o, [bool quack = false]) {
    print("Hello?");
}
"#;
}
