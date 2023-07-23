use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, value},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
};

use crate::dart::{Class, ClassModifier, ClassModifierSet, IdentifierExt};

use super::{common::*, PResult};

pub fn class(s: &str) -> PResult<Class> {
    let (s, modifiers) = terminated(class_modifier_set, spbr)(s)?;
    let (s, name) = terminated(identifier, opt(spbr))(s)?;
    let (s, extends) = opt(terminated(extends, spbr))(s)?;
    let (s, implements) = opt(terminated(implements, spbr))(s)?;
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
}

fn class_modifier_set(s: &str) -> PResult<ClassModifierSet> {
    let (s, modifier) = class_modifier(s)?;

    let modifiers = ClassModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, class_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn class_modifier(s: &str) -> PResult<ClassModifier> {
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

fn extends(s: &str) -> PResult<IdentifierExt> {
    preceded(pair(tag("extends"), spbr), cut(identifier_ext))(s)
}

fn implements(s: &str) -> PResult<Vec<IdentifierExt>> {
    preceded(
        pair(tag("implements"), spbr),
        cut(separated_list1(
            tuple((opt(spbr), tag(","), opt(spbr))),
            identifier_ext,
        )),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extends_test() {
        assert_eq!(
            extends("extends Base "),
            Ok((" ", IdentifierExt::name("Base")))
        );
    }

    #[test]
    fn implements_test() {
        assert_eq!(
            implements("implements A, B, C "),
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
            class("class Record extends Base implements A, B {}"),
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
    fn class_generic_test() {
        assert_eq!(
            class("class Record extends Base<T> implements A<Future<void>> {}"),
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
