use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, value},
    multi::{fold_many0, separated_list1},
    sequence::{pair, preceded, terminated},
    IResult,
};

use crate::dart::{
    Class, ClassMemberModifier, ClassMemberModifierSet, ClassModifier, ClassModifierSet,
};

use super::common::*;

pub fn class(s: &str) -> IResult<&str, Class> {
    let (s, modifiers) = terminated(class_modifier_set, spbr)(s)?;
    let (s, name) = identifier(s)?;
    let (s, extends) = opt(preceded(spbr, extends))(s)?;
    let (s, body) = preceded(opt(spbr), block)(s)?;

    Ok((
        s,
        Class {
            modifiers,
            name,
            extends,
            body,
        },
    ))
}

fn class_modifier_set(s: &str) -> IResult<&str, ClassModifierSet> {
    let (s, modifier) = class_modifier(s)?;

    let modifiers = ClassModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, class_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn class_modifier(s: &str) -> IResult<&str, ClassModifier> {
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

fn extends(s: &str) -> IResult<&str, &str> {
    preceded(pair(tag("extends"), sp), identifier)(s)
}

fn class_property(s: &str) -> IResult<&str, ClassMemberModifierSet> {
    map(separated_list1(sp, class_member_modifier), |modifiers| {
        modifiers.into_iter().collect::<ClassMemberModifierSet>()
    })(s)
}

fn class_member_modifier(s: &str) -> IResult<&str, ClassMemberModifier> {
    alt((
        value(ClassMemberModifier::External, tag("external")),
        value(ClassMemberModifier::Static, tag("static")),
        value(ClassMemberModifier::Const, tag("const")),
        value(ClassMemberModifier::Final, tag("final")),
        value(ClassMemberModifier::Late, tag("late")),
        value(ClassMemberModifier::Covariant, tag("covariant")),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_modifiers_test() {
        assert_eq!(
            class_modifier_set("abstract class"),
            Ok((
                "",
                ClassModifierSet::from_iter(
                    [ClassModifier::Abstract, ClassModifier::Class].into_iter()
                )
            ))
        );
    }

    #[test]
    fn extends_test() {
        assert_eq!(extends("extends Base "), Ok((" ", "Base")));
    }

    #[test]
    fn class_test() {
        assert_eq!(
            class("class Record extends Base {}"),
            Ok((
                "",
                Class {
                    modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                    name: "Record",
                    extends: Some("Base"),
                    body: "{}"
                }
            ))
        );
    }
}
