mod annotation;
mod class;
mod comment;
mod common;
mod directive;
mod enum_ty;
mod expr;
mod extension;
mod func_call;
mod func_like;
mod maybe_required;
mod meta;
mod string;
mod ty;
mod type_params;
mod typedef;
mod var;

use std::str;

use nom::{
    branch::alt,
    combinator::{eof, opt},
    error::{ContextError, ParseError},
    multi::many0,
    sequence::{pair, preceded, terminated},
    Parser,
};

use crate::{
    dart::{meta::WithMeta, *},
    parser::class::class,
};

use self::{
    annotation::annotation,
    comment::comment,
    common::{spbr, spbrc},
    directive::directive,
    enum_ty::enum_ty,
    extension::extension,
    func_like::func_like,
    meta::with_meta,
    typedef::typedef,
    var::var,
};

type PResult<'s, T, E> = Result<(&'s str, T), nom::Err<E>>;

pub fn parse<'s, E>(s: &'s str) -> PResult<Vec<WithMeta<Dart>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        opt(spbr),
        terminated(
            many0(terminated(with_meta(dart_item), opt(spbr))),
            pair(opt(spbrc), eof),
        ),
    )(s)
}

fn dart_item<'s, E>(s: &'s str) -> PResult<Dart, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        directive.map(Dart::Directive),
        typedef.map(Dart::TypeDef),
        var.map(Dart::Var),
        func_like.map(Dart::FuncLike),
        class.map(Dart::Class),
        enum_ty.map(Dart::Enum),
        extension.map(Dart::Extension),
    ))(s)
}

#[cfg(test)]
mod tests {

    use nom::error::VerboseError;

    use crate::dart::{
        class::{ClassMember, ClassModifier, ClassModifierSet, Constructor},
        comment::Comment,
        directive::{Directive, Import},
        func_like::{
            Func, FuncBody, FuncBodyContent, FuncModifierSet, FuncParam, FuncParamModifierSet,
            FuncParams, FuncParamsExtra,
        },
        meta::Meta,
        ty::Type,
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
                    WithMeta::value(Dart::Directive(Directive::Import(Import::target(
                        "dart:math"
                    )))),
                    WithMeta::value(Dart::Directive(Directive::Import(Import::target_as(
                        "package:path/path.dart",
                        "p"
                    )))),
                    WithMeta::value(Dart::Directive(Directive::Part("types.g.dart"))),
                    WithMeta::new(
                        vec![
                            Meta::Comment(Comment::SingleLine("// A comment\n")),
                            Meta::Comment(Comment::MultiLine("/*\nAnother comment\n*/"))
                        ],
                        Dart::Var(Var {
                            modifiers: VarModifierSet::from_iter([VarModifier::Const]),
                            var_type: None,
                            name: "category",
                            initializer: Some(Expr::String("mixed bag")),
                        })
                    ),
                    WithMeta::value(Dart::Var(Var {
                        modifiers: VarModifierSet::from_iter([
                            VarModifier::Late,
                            VarModifier::Final
                        ]),
                        var_type: Some(Type::NotFunc(NotFuncType::name("int"))),
                        name: "crash_count",
                        initializer: None,
                    })),
                    WithMeta::value(Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Base",
                        type_params: Vec::new(),
                        extends: None,
                        with: Vec::new(),
                        implements: Vec::default(),
                        mixin_on: Vec::default(),
                        body: vec![
                            WithMeta::value(ClassMember::Constructor(Constructor {
                                modifier: None,
                                name: "Base",
                                params: FuncParams {
                                    positional_req: vec![WithMeta::value(FuncParam {
                                        modifiers: FuncParamModifierSet::default(),
                                        param_type: None,
                                        name: "this.id",
                                        initializer: None
                                    })],
                                    extra: None,
                                },
                                body: None,
                            })),
                            WithMeta::value(ClassMember::Var(Var {
                                modifiers: VarModifierSet::from_iter([VarModifier::Final]),
                                var_type: Some(Type::NotFunc(NotFuncType::name("String"))),
                                name: "id",
                                initializer: None,
                            })),
                        ],
                    })),
                    WithMeta::new(
                        vec![Meta::Annotation(Annotation::Ident("immutable"))],
                        Dart::Class(Class {
                            modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                            name: "Record",
                            type_params: vec![TypeParam {
                                name: "T",
                                extends: None
                            }],
                            extends: Some(NotFuncType::name("Base")),
                            with: Vec::new(),
                            implements: vec![
                                NotFuncType {
                                    name: "A",
                                    type_args: vec![
                                        Type::NotFunc(NotFuncType {
                                            name: "Future",
                                            type_args: vec![Type::NotFunc(NotFuncType::name(
                                                "void"
                                            ))],
                                            is_nullable: false,
                                        }),
                                        Type::NotFunc(NotFuncType {
                                            name: "B",
                                            type_args: Vec::default(),
                                            is_nullable: true,
                                        }),
                                    ],
                                    is_nullable: false,
                                },
                                NotFuncType::name("C")
                            ],
                            mixin_on: Vec::default(),
                            body: vec![WithMeta::value(ClassMember::Var(Var {
                                modifiers: VarModifierSet::default(),
                                var_type: Some(Type::NotFunc(NotFuncType::name("String"))),
                                name: "name",
                                initializer: None,
                            })),],
                        })
                    ),
                    WithMeta::value(Dart::FuncLike(FuncLike::Func(Func {
                        modifiers: FuncModifierSet::default(),
                        return_type: Type::NotFunc(NotFuncType {
                            name: "Map",
                            type_args: vec![
                                Type::NotFunc(NotFuncType::name("String")),
                                Type::NotFunc(NotFuncType {
                                    name: "Object",
                                    type_args: Vec::new(),
                                    is_nullable: true,
                                })
                            ],
                            is_nullable: false,
                        }),
                        name: "_recordToJson",
                        type_params: Vec::new(),
                        params: FuncParams {
                            positional_req: vec![WithMeta::value(FuncParam {
                                modifiers: FuncParamModifierSet::default(),
                                param_type: Some(Type::NotFunc(NotFuncType::name("Record"))),
                                name: "o",
                                initializer: None
                            })],
                            extra: Some(FuncParamsExtra::PositionalOpt(vec![WithMeta::value(
                                FuncParam {
                                    modifiers: FuncParamModifierSet::default(),
                                    param_type: Some(Type::NotFunc(NotFuncType::name("bool"))),
                                    name: "quack",
                                    initializer: Some(Expr::Ident("false"))
                                }
                            )]))
                        },
                        body: Some(FuncBody {
                            modifier: None,
                            content: FuncBodyContent::Block("{\n    print(\"Hello?\");\n}")
                        })
                    }))),
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

@immutable
class Record<T> extends Base implements A<Future<void>, B?>, C {
  String name;
}

Map<String, Object?> _recordToJson(Record o, [bool quack = false]) {
    print("Hello?");
}
"#;
}
