use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, value},
    multi::fold_many0,
    sequence::{pair, preceded, tuple},
    Parser,
};

use crate::dart::{MemberModifier, MemberModifierSet, Variable};

use super::{
    common::{identifier, identifier_ext, spbr},
    expr::expr,
    PResult,
};

pub fn member_var(s: &str) -> PResult<Variable> {
    tuple((
        alt((
            member_modifier_set,
            value(MemberModifierSet::default(), tag("var")),
        )),
        alt((
            // A type followed by a name
            pair(
                preceded(spbr, identifier_ext).map(Some),
                preceded(opt(spbr), identifier),
            ),
            // Just a name
            preceded(spbr, identifier).map(|id| (None, id)),
        )),
        // An initializer
        opt(preceded(tuple((opt(spbr), tag("="), opt(spbr))), expr)),
        preceded(opt(spbr), tag(";")),
    ))
    .map(|(modifiers, (var_type, name), initializer, _)| Variable {
        modifiers,
        var_type,
        name,
        initializer,
    })
    .parse(s)
}

fn member_modifier_set(s: &str) -> PResult<MemberModifierSet> {
    let (s, modifier) = member_modifier(s)?;

    let modifiers = MemberModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, member_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn member_modifier(s: &str) -> PResult<MemberModifier> {
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
    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn member_var_test() {
        assert_eq!(
            member_var("final String? name; "),
            Ok((
                " ",
                Variable {
                    modifiers: MemberModifierSet::from_iter([MemberModifier::Final]),
                    var_type: Some(IdentifierExt {
                        name: "String",
                        type_args: Vec::default(),
                        is_nullable: true,
                    }),
                    name: "name",
                    initializer: None
                }
            ))
        );
    }

    #[test]
    fn member_var_init() {
        assert_eq!(
            member_var("static const type = \"type\"; "),
            Ok((
                " ",
                Variable {
                    modifiers: MemberModifierSet::from_iter([
                        MemberModifier::Static,
                        MemberModifier::Const,
                    ]),
                    var_type: None,
                    name: "type",
                    initializer: Some("\"type\""),
                }
            ))
        );
    }

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
