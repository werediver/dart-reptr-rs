use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ContextError, ParseError},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::directive::{Directive, Export, Filter, Import, PartOf};

use super::{
    common::{sep_list, spbr, spbrc, SepMode},
    string::string,
    ty::identifier,
    PResult,
};

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

fn export<'s, E>(s: &'s str) -> PResult<Export, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "export",
        preceded(
            pair(tag("export"), spbr),
            cut(terminated(
                pair(terminated(string, opt(spbr)), import_filters),
                tag(";"),
            )),
        )
        .map(|(target, filters)| Export { target, filters }),
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
                    import_filters,
                )),
                tag(";"),
            )),
        )
        .map(|(target, prefix, filters)| Import {
            target,
            prefix,
            filters,
        }),
    )
    .parse(s)
}

fn import_filters<'s, E>(s: &'s str) -> PResult<Vec<Filter>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "import_filters",
        sep_list(
            0,
            SepMode::NoTrailing,
            spbrc,
            alt((show_clause.map(Filter::Show), hide_clause.map(Filter::Hide))),
        ),
    )(s)
}

fn show_clause<'s, E>(s: &'s str) -> PResult<Vec<&str>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("show"), spbrc),
        separated_list1(tuple((opt(spbrc), tag(","), opt(spbrc))), identifier),
    )(s)
}

fn hide_clause<'s, E>(s: &'s str) -> PResult<Vec<&str>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    preceded(
        pair(tag("hide"), spbrc),
        separated_list1(tuple((opt(spbrc), tag(","), opt(spbrc))), identifier),
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
            Ok((
                "x",
                Export {
                    target: "src/utils.dart",
                    filters: Vec::new()
                }
            ))
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
                    filters: vec![Filter::Show(vec!["join"])],
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
                    filters: vec![Filter::Hide(vec!["join", "basename"])],
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
                    filters: vec![
                        Filter::Show(vec!["join", "basename"]),
                        Filter::Hide(vec!["dirname"])
                    ],
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
                    filters: vec![
                        Filter::Hide(vec!["dirname"]),
                        Filter::Show(vec!["join", "basename"])
                    ],
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
