use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt, success},
    error::{context, ContextError, ParseError},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::directive::{Directive, Import, PartOf};

use super::{common::spbr, identifier::identifier, string::string, PResult};

pub fn directive<'s, E>(s: &'s str) -> PResult<Directive, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "directive",
        alt((
            export.map(Directive::Export),
            import.map(Directive::Import),
            part_of.map(Directive::PartOf),
            part.map(Directive::Part),
        )),
    )(s)
}

fn export<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "export",
        preceded(
            pair(tag("export"), spbr),
            cut(terminated(terminated(string, opt(spbr)), tag(";"))),
        ),
    )(s)
}

fn import<'s, E>(s: &'s str) -> PResult<Import, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "import",
        preceded(
            pair(tag("import"), spbr),
            cut(terminated(
                tuple((
                    terminated(string, opt(spbr)),
                    opt(preceded(
                        pair(tag("as"), spbr),
                        terminated(identifier, opt(spbr)),
                    )),
                    alt((
                        terminated(
                            pair(show_clause, opt(preceded(opt(spbr), hide_clause)))
                                .map(|(show, hide)| (Some(show), hide)),
                            opt(spbr),
                        ),
                        terminated(
                            pair(hide_clause, opt(preceded(opt(spbr), show_clause)))
                                .map(|(hide, show)| (show, Some(hide))),
                            opt(spbr),
                        ),
                        success((None, None)),
                    )),
                )),
                tag(";"),
            )),
        )
        .map(|(target, prefix, (show, hide))| Import {
            target,
            prefix,
            show: show.unwrap_or(Vec::new()),
            hide: hide.unwrap_or(Vec::new()),
        }),
    )
    .parse(s)
}

fn show_clause<'s, E>(s: &'s str) -> PResult<Vec<&str>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("show"), spbr),
        separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), identifier),
    )(s)
}

fn hide_clause<'s, E>(s: &'s str) -> PResult<Vec<&str>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("hide"), spbr),
        separated_list1(tuple((opt(spbr), tag(","), opt(spbr))), identifier),
    )(s)
}

fn part<'s, E>(s: &'s str) -> PResult<&str, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "part",
        preceded(
            pair(tag("part"), spbr),
            cut(terminated(string, pair(opt(spbr), tag(";")))),
        ),
    )(s)
}

fn part_of<'s, E>(s: &'s str) -> PResult<PartOf, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "part_of",
        preceded(
            tuple((tag("part"), spbr, tag("of"), spbr)),
            cut(terminated(
                alt((string.map(PartOf::LibPath), identifier.map(PartOf::LibName))),
                pair(opt(spbr), tag(";")),
            )),
        ),
    )(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use super::*;

    #[test]
    fn export_test() {
        assert_eq!(
            export::<VerboseError<_>>("export 'src/utils.dart';x"),
            Ok(("x", "src/utils.dart"))
        );
    }

    #[test]
    fn import_test() {
        assert_eq!(
            import::<VerboseError<_>>("import 'dart:math';x"),
            Ok(("x", Import::target("dart:math")))
        );
    }

    #[test]
    fn import_as_test() {
        assert_eq!(
            import::<VerboseError<_>>("import 'package:path/path.dart' as p;x"),
            Ok(("x", Import::target_as("package:path/path.dart", "p")))
        );
    }

    #[test]
    fn import_show_test() {
        assert_eq!(
            import::<VerboseError<_>>("import 'package:path/path.dart' show join;x"),
            Ok((
                "x",
                Import {
                    target: "package:path/path.dart",
                    prefix: None,
                    show: vec!["join"],
                    hide: Vec::default(),
                }
            ))
        );
    }

    #[test]
    fn import_hide_test() {
        assert_eq!(
            import::<VerboseError<_>>("import 'package:path/path.dart' hide join, basename;x"),
            Ok((
                "x",
                Import {
                    target: "package:path/path.dart",
                    prefix: None,
                    show: Vec::default(),
                    hide: vec!["join", "basename"],
                }
            ))
        );
    }

    #[test]
    fn import_as_show_hide_test() {
        assert_eq!(
            import::<VerboseError<_>>(
                "import 'package:path/path.dart' as p show join, basename hide dirname;x"
            ),
            Ok((
                "x",
                Import {
                    target: "package:path/path.dart",
                    prefix: Some("p"),
                    show: vec!["join", "basename"],
                    hide: vec!["dirname"],
                }
            ))
        );
    }

    #[test]
    fn import_as_hide_show_test() {
        assert_eq!(
            import::<VerboseError<_>>(
                "import 'package:path/path.dart' as p hide dirname show join, basename;x"
            ),
            Ok((
                "x",
                Import {
                    target: "package:path/path.dart",
                    prefix: Some("p"),
                    show: vec!["join", "basename"],
                    hide: vec!["dirname"],
                }
            ))
        );
    }

    #[test]
    fn part_test() {
        assert_eq!(
            part::<VerboseError<_>>("part '../library.dart';x"),
            Ok(("x", "../library.dart"))
        );
    }

    #[test]
    fn part_of_path_test() {
        assert_eq!(
            part_of::<VerboseError<_>>("part of '../library.dart';x"),
            Ok(("x", PartOf::LibPath("../library.dart")))
        );
    }

    #[test]
    fn part_of_name_test() {
        assert_eq!(
            part_of::<VerboseError<_>>("part of library;x"),
            Ok(("x", PartOf::LibName("library")))
        );
    }
}
