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
    Class, NotFuncType, WithMeta,
};

use super::{
    common::*,
    expr::expr,
    func_like::{func_body_content, func_like, func_params},
    meta::with_meta,
    ty::{identifier, not_func_type},
    type_params::type_params,
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
            opt(terminated(type_params, opt(spbr))),
            opt(terminated(extends_clause, opt(spbr))),
            opt(terminated(with_clause, opt(spbr))),
            opt(terminated(implements_clause, opt(spbr))),
            opt(terminated(mixin_on_clause, opt(spbr))),
            class_body,
        ))
        .map(
            |(modifiers, name, type_params, extends, with, implements, on, body)| Class {
                modifiers,
                name,
                type_params: type_params.unwrap_or(Vec::new()),
                extends,
                with: with.unwrap_or(Vec::new()),
                implements: implements.unwrap_or(Vec::new()),
                mixin_on: on.unwrap_or(Vec::new()),
                body,
            },
        ),
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

fn extends_clause<'s, E>(s: &'s str) -> PResult<NotFuncType, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "extends_clause",
        preceded(pair(tag("extends"), spbr), cut(not_func_type)),
    )(s)
}

pub fn implements_clause<'s, E>(s: &'s str) -> PResult<Vec<NotFuncType>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "implements_clause",
        preceded(
            pair(tag("implements"), spbrc),
            cut(separated_list1(
                pair(tag(","), opt(spbrc)),
                terminated(not_func_type, opt(spbrc)),
            )),
        ),
    )(s)
}

pub fn mixin_on_clause<'s, E>(s: &'s str) -> PResult<Vec<NotFuncType>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "mixin_on_clause",
        preceded(
            pair(tag("on"), spbrc),
            cut(separated_list1(
                pair(tag(","), opt(spbrc)),
                terminated(not_func_type, opt(spbrc)),
            )),
        ),
    )(s)
}

pub fn with_clause<'s, E>(s: &'s str) -> PResult<Vec<NotFuncType>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "with_clause",
        preceded(
            pair(tag("with"), spbr),
            cut(separated_list1(
                tuple((opt(spbr), tag(","), opt(spbr))),
                not_func_type,
            )),
        ),
    )(s)
}

fn class_body<'s, E>(s: &'s str) -> PResult<Vec<WithMeta<ClassMember>>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "class_body",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                many0(terminated(with_meta(class_member), opt(spbr))),
                pair(opt(spbrc), tag("}")),
            )),
        ),
    )(s)
}

pub fn class_member<'s, E>(s: &'s str) -> PResult<ClassMember, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        constructor.map(ClassMember::Constructor),
        var.map(ClassMember::Var),
        func_like.map(ClassMember::FuncLike),
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
        value(ConstructorModifier::External, tag("external")),
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
        func_like::{FuncBodyContent, FuncParam, FuncParamModifierSet, FuncParams},
        meta::Meta,
        ty::Type,
        var::VarModifierSet,
        Annotation, TypeParam, Var,
    };

    use super::*;

    #[test]
    fn extends_test() {
        assert_eq!(
            extends_clause::<VerboseError<_>>("extends Base "),
            Ok((" ", NotFuncType::name("Base")))
        );
    }

    #[test]
    fn with_test() {
        assert_eq!(
            with_clause::<VerboseError<_>>("with Salt, Pepper<Black> "),
            Ok((
                " ",
                vec![
                    NotFuncType::name("Salt"),
                    NotFuncType {
                        name: "Pepper",
                        type_args: vec![Type::NotFunc(NotFuncType::name("Black"))],
                        is_nullable: false,
                    }
                ]
            ))
        );
    }

    #[test]
    fn implements_test() {
        assert_eq!(
            implements_clause::<VerboseError<_>>("implements A, B, C x"),
            Ok((
                "x",
                vec![
                    NotFuncType::name("A"),
                    NotFuncType::name("B"),
                    NotFuncType::name("C")
                ]
            ))
        );
    }

    #[test]
    fn class_test() {
        assert_eq!(
            class::<VerboseError<_>>(
                "class Record<T, U extends Object> extends Base implements A, B {} "
            ),
            Ok((
                " ",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    type_params: vec![
                        TypeParam {
                            name: "T",
                            extends: None
                        },
                        TypeParam {
                            name: "U",
                            extends: Some(Type::NotFunc(NotFuncType::name("Object")))
                        },
                    ],
                    extends: Some(NotFuncType::name("Base")),
                    with: Vec::new(),
                    implements: vec![NotFuncType::name("A"), NotFuncType::name("B")],
                    mixin_on: Vec::default(),
                    body: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn class_property_test() {
        assert_eq!(
            class::<VerboseError<_>>("class Record {\n  @override\n  String id;\n}"),
            Ok((
                "",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    type_params: Vec::new(),
                    extends: None,
                    with: Vec::new(),
                    implements: Vec::new(),
                    mixin_on: Vec::default(),
                    body: vec![WithMeta::new(
                        vec![Meta::Annotation(Annotation::Ident("override"))],
                        ClassMember::Var(Var {
                            modifiers: VarModifierSet::default(),
                            var_type: Some(Type::NotFunc(NotFuncType::name("String"))),
                            name: "id",
                            initializer: None,
                        }),
                    )],
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
                    type_params: Vec::new(),
                    extends: Some(NotFuncType {
                        name: "Base",
                        type_args: vec![Type::NotFunc(NotFuncType::name("T"))],
                        is_nullable: false,
                    }),
                    with: Vec::new(),
                    implements: vec![NotFuncType {
                        name: "A",
                        type_args: vec![Type::NotFunc(NotFuncType {
                            name: "Future",
                            type_args: vec![Type::NotFunc(NotFuncType::name("void"))],
                            is_nullable: false,
                        })],
                        is_nullable: false,
                    }],
                    mixin_on: Vec::default(),
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
                        positional_req: vec![WithMeta::value(FuncParam {
                            modifiers: FuncParamModifierSet::default(),
                            param_type: None,
                            name: "this.id",
                            initializer: None,
                        })],
                        extra: None
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
