use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success, value},
    error::{context, ContextError, ParseError},
    multi::fold_many0,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{Var, VarModifier, VarModifierSet};

use super::{
    common::{identifier, identifier_ext, spbr},
    expr::expr,
    PResult,
};

pub fn var<'s, E>(s: &'s str) -> PResult<Var, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "var",
        tuple((
            alt((
                terminated(var_modifier_set, spbr),
                success(VarModifierSet::default()),
            )),
            opt(terminated(tag("var"), spbr)),
            alt((
                // A type followed by a name
                pair(
                    terminated(identifier_ext, opt(spbr)).map(Some),
                    terminated(identifier, opt(spbr)),
                ),
                // Just a name
                terminated(identifier, opt(spbr)).map(|id| (None, id)),
            )),
            // An initializer
            opt(preceded(
                pair(tag("="), opt(spbr)),
                cut(terminated(expr, opt(spbr))),
            )),
            tag(";"),
        ))
        .map(|(modifiers, _, (var_type, name), initializer, _)| Var {
            modifiers,
            var_type,
            name,
            initializer,
        }),
    )
    .parse(s)
}

fn var_modifier_set<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<VarModifierSet, E> {
    let (s, modifier) = var_modifier(s)?;

    let modifiers = VarModifierSet::from_iter([modifier]);

    fold_many0(
        preceded(spbr, var_modifier),
        move || modifiers,
        |modifiers, modifier| modifiers.with(modifier),
    )(s)
}

fn var_modifier<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<VarModifier, E> {
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
    use nom::error::VerboseError;

    use crate::dart::IdentifierExt;

    use super::*;

    #[test]
    fn var_test() {
        assert_eq!(
            var::<VerboseError<_>>("final String? name; "),
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
    fn var_init() {
        assert_eq!(
            var::<VerboseError<_>>("static const type = \"type\"; "),
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
    fn var_mut_no_type_init() {
        assert_eq!(
            var::<VerboseError<_>>("var i = 0; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::default(),
                    var_type: None,
                    name: "i",
                    initializer: Some("0"),
                }
            ))
        );
    }

    #[test]
    fn var_mut_type_init() {
        assert_eq!(
            var::<VerboseError<_>>("double x = 0; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::default(),
                    var_type: Some(IdentifierExt::name("double")),
                    name: "x",
                    initializer: Some("0"),
                }
            ))
        );
    }

    #[test]
    fn var_mut_type() {
        assert_eq!(
            var::<VerboseError<_>>("double x; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::default(),
                    var_type: Some(IdentifierExt::name("double")),
                    name: "x",
                    initializer: None,
                }
            ))
        );
    }

    #[test]
    fn var_late_final_type_type() {
        assert_eq!(
            var::<VerboseError<_>>("late final int crash_count; "),
            Ok((
                " ",
                Var {
                    modifiers: VarModifierSet::from_iter([VarModifier::Late, VarModifier::Final]),
                    var_type: Some(IdentifierExt::name("int")),
                    name: "crash_count",
                    initializer: None,
                }
            ))
        );
    }

    #[test]
    fn var_modifier_set_test() {
        assert_eq!(
            var_modifier_set::<VerboseError<_>>("late final "),
            Ok((
                " ",
                VarModifierSet::from_iter([VarModifier::Late, VarModifier::Final].into_iter())
            ))
        );
    }
}
