use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::separated_list1,
    IResult,
};

use crate::dart::{
    Class, ClassMemberModifier, ClassMemberModifierSet, ClassModifier, ClassModifierSet,
};

use super::common::*;

pub fn class(s: &str) -> IResult<&str, Class> {
    let (s, modifiers) = class_modifier_set(s)?;
    let (s, _) = sp(s)?;
    let (s, name) = identifier(s)?;
    let (s, _) = sp(s).unwrap_or((s, ""));
    let (s, body) = block(s)?;

    Ok((
        s,
        Class {
            modifiers,
            name,
            body,
        },
    ))
}

fn class_modifier_set(s: &str) -> IResult<&str, ClassModifierSet> {
    map(separated_list1(sp, class_modifier), |modifiers| {
        modifiers.into_iter().collect::<ClassModifierSet>()
    })(s)
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
}
