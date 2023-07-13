mod class;
mod common;
mod string;

use std::str;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{eof, map, recognize},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{
    dart::*,
    parser::{class::class, common::*},
};

use self::string::string_simple;

pub fn parse(s: &str) -> IResult<&str, Vec<Dart>> {
    let (s, items) = many0(alt((
        map(alt((spbr, comment)), Dart::Verbatim),
        map(class, Dart::Class),
    )))(s)?;
    let (s, _) = eof(s)?;

    Ok((s, items))
}

fn comment(s: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("//"), is_not("\r\n"), br)))(s)
}

fn import(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("import")(s)?;
    let (s, _) = sp(s)?;
    let (s, _) = string_simple(s)?;
    let (s, _) = sp(s)?;
    let (s, _) = tag(";")(s)?;

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_test() {
        let s = "// A comment\nx";
        assert_eq!(comment(s), Ok(("x", "// A comment\n")));
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
