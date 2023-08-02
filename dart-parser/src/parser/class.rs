use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, recognize, value},
    error::{context, ContextError, ParseError},
    multi::{fold_many0, many0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{
    class::{ClassMember, ClassModifier, ClassModifierSet, Constructor, ConstructorModifier},
    Class, IdentifierExt,
};

use super::{
    comment::comment,
    common::*,
    expr::expr,
    func::{func, func_body_content, func_params},
    var::var,
    PResult,
};

pub fn class<'s, E>(s: &'s str) -> PResult<Class, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "class",
        tuple((
            terminated(class_modifier_set, spbr),
            terminated(identifier, opt(spbr)),
            opt(terminated(extends_clause, opt(spbr))),
            opt(terminated(implements_clause, opt(spbr))),
            class_body,
        ))
        .map(|(modifiers, name, extends, implements, body)| Class {
            modifiers,
            name,
            extends,
            implements: implements.unwrap_or(Vec::default()),
            body,
        }),
    )(s)
}

fn class_modifier_set<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<ClassModifierSet, E> {
    let (s, modifier) = class_modifier(s)?;

    let modifiers = ClassModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, class_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn class_modifier<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<ClassModifier, E> {
    alt((
        value(ClassModifier::Abstract, tag("abstract")),
        value(ClassModifier::Base, tag("base")),
        value(ClassModifier::Class, tag("class")),
        value(ClassModifier::Final, tag("final")),
        value(ClassModifier::Interface, tag("interface")),
        value(ClassModifier::Sealed, tag("sealed")),
        value(ClassModifier::Mixin, tag("mixin")),
    ))(s)
}

fn extends_clause<'s, E>(s: &'s str) -> PResult<IdentifierExt, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "extends_clause",
        preceded(pair(tag("extends"), spbr), cut(identifier_ext)),
    )(s)
}

pub fn implements_clause<'s, E>(s: &'s str) -> PResult<Vec<IdentifierExt>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "implements_clause",
        preceded(
            pair(tag("implements"), spbr),
            cut(separated_list1(
                tuple((opt(spbr), tag(","), opt(spbr))),
                identifier_ext,
            )),
        ),
    )(s)
}

fn class_body<'s, E>(s: &'s str) -> PResult<Vec<ClassMember>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "class_body",
        preceded(tag("{"), terminated(many0(class_member), tag("}"))),
    )(s)
}

fn class_member<'s, E>(s: &'s str) -> PResult<ClassMember, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        spbr.map(ClassMember::Verbatim),
        comment.map(ClassMember::Comment),
        constructor.map(ClassMember::Constructor),
        var.map(ClassMember::Var),
        func.map(ClassMember::Func),
    ))(s)
}

fn constructor<'s, E>(s: &'s str) -> PResult<Constructor, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "constructor",
        tuple((
            opt(terminated(constructor_modifier, spbr)),
            terminated(identifier, opt(spbr)),
            terminated(func_params, opt(spbr)),
            opt(terminated(constructor_initializer_list, opt(spbr))),
            alt((func_body_content.map(Some), tag(";").map(|_| None))),
        ))
        .map(|(modifier, name, params, _, body)| Constructor {
            modifier,
            name,
            params,
            body,
        }),
    )(s)
}

fn constructor_modifier<'s, E>(s: &'s str) -> PResult<ConstructorModifier, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        value(ConstructorModifier::Const, tag("const")),
        value(ConstructorModifier::Factory, tag("factory")),
    ))(s)
}

fn constructor_initializer_list<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag(":"), opt(spbr)),
        recognize(separated_list1(
            tuple((opt(spbr), tag(","), opt(spbr))),
            expr,
        )),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{
        func::{FuncBodyContent, FuncParam, FuncParamModifierSet, FuncParams},
        var::VarModifierSet,
        Var,
    };

    use super::*;

    #[test]
    fn extends_test() {
        assert_eq!(
            extends_clause::<VerboseError<_>>("extends Base "),
            Ok((" ", IdentifierExt::name("Base")))
        );
    }

    #[test]
    fn implements_test() {
        assert_eq!(
            implements_clause::<VerboseError<_>>("implements A, B, C "),
            Ok((
                " ",
                vec![
                    IdentifierExt::name("A"),
                    IdentifierExt::name("B"),
                    IdentifierExt::name("C")
                ]
            ))
        );
    }

    #[test]
    fn class_test() {
        assert_eq!(
            class::<VerboseError<_>>("class Record extends Base implements A, B {}"),
            Ok((
                "",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    extends: Some(IdentifierExt::name("Base")),
                    implements: vec![IdentifierExt::name("A"), IdentifierExt::name("B")],
                    body: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn class_property_test() {
        assert_eq!(
            class::<VerboseError<_>>("class Record {\n  String id;\n}"),
            Ok((
                "",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    extends: None,
                    implements: Vec::new(),
                    body: vec![
                        ClassMember::Verbatim("\n  "),
                        ClassMember::Var(Var {
                            modifiers: VarModifierSet::default(),
                            var_type: Some(IdentifierExt::name("String")),
                            name: "id",
                            initializer: None,
                        }),
                        ClassMember::Verbatim("\n"),
                    ],
                }
            ))
        );
    }

    #[test]
    fn class_generic_base_test() {
        assert_eq!(
            class::<VerboseError<_>>("class Record extends Base<T> implements A<Future<void>> {}"),
            Ok((
                "",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    extends: Some(IdentifierExt {
                        name: "Base",
                        type_args: vec![IdentifierExt::name("T")],
                        is_nullable: false,
                    }),
                    implements: vec![IdentifierExt {
                        name: "A",
                        type_args: vec![IdentifierExt {
                            name: "Future",
                            type_args: vec![IdentifierExt::name("void")],
                            is_nullable: false,
                        }],
                        is_nullable: false,
                    }],
                    body: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn constructor_basic_test() {
        assert_eq!(
            constructor::<VerboseError<_>>("Record(); "),
            Ok((
                " ",
                Constructor {
                    modifier: None,
                    name: "Record",
                    params: FuncParams::default(),
                    body: None,
                }
            ))
        );
    }

    #[test]
    fn constructor_param_this_test() {
        assert_eq!(
            constructor::<VerboseError<_>>("Record(this.id); "),
            Ok((
                " ",
                Constructor {
                    modifier: None,
                    name: "Record",
                    params: FuncParams {
                        positional: vec![FuncParam {
                            is_required: true,
                            modifiers: FuncParamModifierSet::default(),
                            param_type: None,
                            name: "this.id",
                            initializer: None,
                        }],
                        named: Vec::new(),
                    },
                    body: None,
                }
            ))
        );
    }

    #[test]
    fn constructor_initializer_list_test() {
        assert_eq!(
            constructor::<VerboseError<_>>(
                "const Record(): assert(() { print('+1'); }()), super(null); "
            ),
            Ok((
                " ",
                Constructor {
                    modifier: Some(ConstructorModifier::Const),
                    name: "Record",
                    params: FuncParams::default(),
                    body: None,
                }
            ))
        );
    }

    #[test]
    fn constructor_factory_test() {
        assert_eq!(
            constructor::<VerboseError<_>>("factory Record.default() { print('+1'); } "),
            Ok((
                " ",
                Constructor {
                    modifier: Some(ConstructorModifier::Factory),
                    name: "Record.default",
                    params: FuncParams::default(),
                    body: Some(FuncBodyContent::Block("{ print('+1'); }")),
                }
            ))
        );
    }
}
