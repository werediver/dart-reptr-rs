use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, value},
    error::{context, ContextError, ParseError},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};

use crate::dart::{
    class::{ClassModifier, ClassModifierSet},
    Class, IdentifierExt,
};

use super::{common::*, scope::block, PResult};

pub fn class<'s, E>(s: &'s str) -> PResult<Class, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context("class", |s| {
        let (s, modifiers) = terminated(class_modifier_set, spbr)(s)?;
        let (s, name) = terminated(identifier, opt(spbr))(s)?;
        let (s, extends) = opt(terminated(extends_clause, spbr))(s)?;
        let (s, implements) = opt(terminated(implements_clause, spbr))(s)?;
        let (s, body) = block(s)?;

        Ok((
            s,
            Class {
                modifiers,
                name,
                extends,
                implements: implements.unwrap_or(Vec::default()),
                body,
            },
        ))
    })(s)
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

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

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
                    body: "{}"
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
                    body: "{\n  String id;\n}"
                }
            ))
        );
    }

    #[test]
    fn class_generic_test() {
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
                    body: "{}"
                }
            ))
        );
    }
}
