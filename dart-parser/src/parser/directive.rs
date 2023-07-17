use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    combinator::{cut, opt},
    multi::separated_list1,
    sequence::{pair, preceded, terminated, tuple},
    IResult, Parser,
};

use crate::dart::directive::{Directive, Import, PartOf};

use super::{
    common::{identifier, spbr},
    string::string_simple,
};

pub fn directive(s: &str) -> IResult<&str, Directive> {
    alt((
        export.map(Directive::Export),
        import.map(Directive::Import),
        part_of.map(Directive::PartOf),
        part.map(Directive::Part),
    ))(s)
}

fn export(s: &str) -> IResult<&str, &str> {
    preceded(
        pair(tag("export"), spbr),
        cut(terminated(terminated(string_simple, opt(spbr)), tag(";"))),
    )(s)
}

fn import(s: &str) -> IResult<&str, Import> {
    preceded(
        pair(tag("import"), spbr),
        cut(terminated(
            tuple((
                terminated(string_simple, opt(spbr)),
                opt(preceded(
                    pair(tag("as"), spbr),
                    terminated(identifier, opt(spbr)),
                )),
                permutation((
                    opt(preceded(
                        pair(tag("show"), spbr),
                        separated_list1(
                            pair(tag(","), opt(spbr)),
                            terminated(identifier, opt(spbr)),
                        ),
                    )),
                    opt(preceded(
                        pair(tag("hide"), spbr),
                        separated_list1(
                            pair(tag(","), opt(spbr)),
                            terminated(identifier, opt(spbr)),
                        ),
                    )),
                )),
            )),
            tag(";"),
        )),
    )
    .map(|(target, prefix, (show, hide))| Import {
        target,
        prefix,
        show: show.unwrap_or(Vec::default()),
        hide: hide.unwrap_or(Vec::default()),
    })
    .parse(s)
}

fn part(s: &str) -> IResult<&str, &str> {
    preceded(
        pair(tag("part"), spbr),
        cut(terminated(string_simple, pair(opt(spbr), tag(";")))),
    )(s)
}

fn part_of(s: &str) -> IResult<&str, PartOf> {
    preceded(
        tuple((tag("part"), spbr, tag("of"), spbr)),
        cut(terminated(
            alt((
                string_simple.map(PartOf::LibPath),
                identifier.map(PartOf::LibName),
            )),
            pair(opt(spbr), tag(";")),
        )),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_test() {
        assert_eq!(
            export("export 'src/utils.dart';x"),
            Ok(("x", "src/utils.dart"))
        );
    }

    #[test]
    fn import_test() {
        assert_eq!(
            import("import 'dart:math';x"),
            Ok(("x", Import::target("dart:math")))
        );
    }

    #[test]
    fn import_as_test() {
        assert_eq!(
            import("import 'package:path/path.dart' as p;x"),
            Ok(("x", Import::target_as("package:path/path.dart", "p")))
        );
    }

    #[test]
    fn import_show_test() {
        assert_eq!(
            import("import 'package:path/path.dart' show join;x"),
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
            import("import 'package:path/path.dart' hide join, basename;x"),
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
            import("import 'package:path/path.dart' as p show join, basename hide dirname;x"),
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
            part("part '../library.dart';x"),
            Ok(("x", "../library.dart"))
        );
    }

    #[test]
    fn part_of_path_test() {
        assert_eq!(
            part_of("part of '../library.dart';x"),
            Ok(("x", PartOf::LibPath("../library.dart")))
        );
    }

    #[test]
    fn part_of_name_test() {
        assert_eq!(
            part_of("part of library;x"),
            Ok(("x", PartOf::LibName("library")))
        );
    }
}
