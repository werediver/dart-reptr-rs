use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, value},
    multi::fold_many0,
    sequence::{pair, preceded, tuple},
    Parser,
};

use crate::dart::{Var, VarModifier, VarModifierSet};

use super::{
    common::{identifier, identifier_ext, spbr},
    expr::expr,
    PResult,
};

pub fn var(s: &str) -> PResult<Var> {
    tuple((
        alt((
            var_modifier_set,
            value(VarModifierSet::default(), tag("var")),
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
    .map(|(modifiers, (var_type, name), initializer, _)| Var {
        modifiers,
        var_type,
        name,
        initializer,
    })
    .parse(s)
}

fn var_modifier_set(s: &str) -> PResult<VarModifierSet> {
    let (s, modifier) = var_modifier(s)?;

    let modifiers = VarModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, var_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn var_modifier(s: &str) -> PResult<VarModifier> {
    alt((
        value(VarModifier::External, tag("external")),
        value(VarModifier::Static, tag("static")),
        value(VarModifier::Const, tag("const")),
        value(VarModifier::Final, tag("final")),
        value(VarModifier::Late, tag("late")),
        value(VarModifier::Covariant, tag("covariant")),
    ))(s)
}

#[cfg(test)]
mod tests {
    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn member_var_test() {
        assert_eq!(
            var("final String? name; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::from_iter([VarModifier::Final]),
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
            var("static const type = \"type\"; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::from_iter(
                        [VarModifier::Static, VarModifier::Const,]
                    ),
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
            var_modifier_set("late final "),
            Ok((
                " ",
                VarModifierSet::from_iter([VarModifier::Late, VarModifier::Final].into_iter())
            ))
        );
    }
}
