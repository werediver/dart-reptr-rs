use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while, take_while_m_n},
    character::complete::char,
    combinator::{consumed, map, recognize, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::dart::{
    Class, ClassMemberModifier, ClassMemberModifierSet, ClassModifier, ClassModifierSet, Dart,
};

fn parse(s: &str) -> IResult<&str, Vec<Dart>> {
    many0(alt((
        map(alt((spbr, comment)), Dart::Verbatim),
        map(class, Dart::Class),
    )))(s)
}

/// Parse one or more whitespace characters, excluding line breaks.
fn sp(s: &str) -> IResult<&str, &str> {
    is_a(" \t")(s)
}

/// Parse one or more whitespace characters, including line breaks.
fn spbr(s: &str) -> IResult<&str, &str> {
    is_a(" \t\r\n")(s)
}

/// Parse exactly one line break.
fn br(s: &str) -> IResult<&str, &str> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(s)
}

fn comment(s: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("//"), is_not("\r\n"), br)))(s)
}

fn identifier(s: &str) -> IResult<&str, &str> {
    fn is_start_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }

    fn is_part_char(c: char) -> bool {
        is_start_char(c) || c.is_ascii_digit()
    }

    map(
        consumed(pair(
            take_while_m_n(1, 1, is_start_char),
            take_while(is_part_char),
        )),
        |(c, _)| c,
    )(s)
}

fn class(s: &str) -> IResult<&str, Class> {
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

fn block(s: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('{'),
        recognize(many0(alt((is_not("{}"), block)))),
        char('}'),
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
    fn sp_test() {
        let s = "  \n\t\r\nx";
        assert_eq!(spbr(s), Ok(("x", "  \n\t\r\n")));
    }

    #[test]
    fn comment_test() {
        let s = "// A comment\nx";
        assert_eq!(comment(s), Ok(("x", "// A comment\n")));
    }

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
    fn basic() {
        assert_eq!(
            parse(DART_BASIC.trim_start()),
            Ok((
                "",
                vec![
                    Dart::Verbatim("// A comment\n"),
                    Dart::Verbatim("\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Record1",
                        body: "{\n  String field;\n}",
                    }),
                    Dart::Verbatim("\n\n"),
                    Dart::Class(Class {
                        modifiers: ClassModifierSet::from_iter([ClassModifier::Class]),
                        name: "Record2",
                        body: "{\n  String field;\n}",
                    }),
                    Dart::Verbatim("\n")
                ]
            ))
        );
    }

    const DART_BASIC: &str = r#"
// A comment

class Record1 {
  String field;
}

class Record2 {
  String field;
}
"#;
}
