use nom::{
    branch::alt, bytes::complete::tag, combinator::value, multi::fold_many0, sequence::preceded,
    IResult,
};

use crate::dart::{MemberModifier, MemberModifierSet};

use super::common::spbr;

fn member_modifier_set(s: &str) -> IResult<&str, MemberModifierSet> {
    let (s, modifier) = member_modifier(s)?;

    let modifiers = MemberModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, member_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn member_modifier(s: &str) -> IResult<&str, MemberModifier> {
    alt((
        value(MemberModifier::External, tag("external")),
        value(MemberModifier::Static, tag("static")),
        value(MemberModifier::Const, tag("const")),
        value(MemberModifier::Final, tag("final")),
        value(MemberModifier::Late, tag("late")),
        value(MemberModifier::Covariant, tag("covariant")),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_modifier_set_test() {
        assert_eq!(
            member_modifier_set("late final "),
            Ok((
                " ",
                MemberModifierSet::from_iter(
                    [MemberModifier::Late, MemberModifier::Final].into_iter()
                )
            ))
        );
    }
}
