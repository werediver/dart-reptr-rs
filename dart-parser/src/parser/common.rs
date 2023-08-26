use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    error::{ContextError, ParseError},
    multi::{fold_many0, fold_many1},
    InputLength, Parser,
};

use super::PResult;

/// Convert irrecoverable errors from into recoverable.
///
/// For use in shady parsers.
pub fn uncut<P, I, O, E>(mut p: P) -> impl FnMut(I) -> nom::IResult<I, O, E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    move |s| match p.parse(s) {
        result @ Ok(_) => result,
        result @ Err(nom::Err::Incomplete(_)) => result,
        result @ Err(nom::Err::Error(_)) => result,
        // Convert an irrecoverable error into a recoverable
        Err(nom::Err::Failure(e)) => Err(nom::Err::Error(e)),
    }
}

pub fn skip_many0<P, I, O, E>(p: P) -> impl FnMut(I) -> nom::IResult<I, (), E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    fold_many0(p, || {}, |_, _| {})
}

pub fn skip_many1<P, I, O, E>(p: P) -> impl FnMut(I) -> nom::IResult<I, (), E>
where
    P: Parser<I, O, E>,
    I: Clone + InputLength,
    E: ParseError<I>,
{
    fold_many1(p, || {}, |_, _| {})
}

pub fn sep_list<'p, 's, Item, Sep, E, SepP, ItemP>(
    count_min: usize,
    sep_mode: SepMode,
    mut sep: SepP,
    mut item: ItemP,
) -> impl FnMut(&'s str) -> PResult<'s, Vec<Item>, E> + 'p
where
    E: ParseError<&'s str> + ContextError<&'s str>,
    SepP: Parser<&'s str, Sep, E> + 'p,
    ItemP: Parser<&'s str, Item, E> + 'p,
{
    move |mut s| {
        let mut items = Vec::new();
        match item.parse(s) {
            Ok((s_advanced, value)) => {
                items.push(value);
                s = s_advanced;
            }
            Err(e) => {
                if items.len() < count_min {
                    return Err(e);
                } else {
                    return Ok((s, items));
                }
            }
        };
        loop {
            match sep.parse(s) {
                Ok((s_advanced, _)) => match item.parse(s_advanced) {
                    Ok((s_advanced, value)) => {
                        items.push(value);
                        s = s_advanced;
                    }
                    Err(e) => {
                        if items.len() < count_min {
                            return Err(e);
                        } else {
                            return match sep_mode {
                                SepMode::NoTrailing => Ok((s, items)),
                                SepMode::AllowTrailing => Ok((s_advanced, items)),
                            };
                        }
                    }
                },
                Err(e) => {
                    if items.len() < count_min {
                        return Err(e);
                    } else {
                        return Ok((s, items));
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum SepMode {
    NoTrailing,
    AllowTrailing,
}

/// Parse one or more whitespace characters, including line breaks.
pub fn spbr<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<&str, E> {
    is_a(" \t\r\n")(s)
}

/// Parse exactly one line break.
pub fn br<'s, E: ParseError<&'s str>>(s: &'s str) -> PResult<&str, E> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(s)
}

#[cfg(test)]
mod tests {
    use nom::{combinator::opt, error::VerboseError};

    use super::*;

    #[test]
    fn sp_test() {
        let s = "  \n\t\r\nx";
        assert_eq!(spbr::<VerboseError<_>>(s), Ok(("x", "  \n\t\r\n")));
    }

    #[test]
    fn sep_list0_empty_test() {
        assert_eq!(
            sep_list::<_, _, VerboseError<_>, _, _>(0, SepMode::NoTrailing, spbr, tag("x"))("z"),
            Ok(("z", Vec::new()))
        );
    }

    #[test]
    fn sep_list1_empty_test() {
        assert!(
            sep_list::<_, _, VerboseError<_>, _, _>(1, SepMode::NoTrailing, spbr, tag("x"))("z")
                .is_err()
        );
    }

    #[test]
    fn sep_list1_one_test() {
        assert_eq!(
            sep_list::<_, _, VerboseError<_>, _, _>(1, SepMode::NoTrailing, spbr, tag("x"))("xz"),
            Ok(("z", vec!["x"]))
        );
    }

    #[test]
    fn sep_list1_some_test() {
        assert_eq!(
            sep_list::<_, _, VerboseError<_>, _, _>(1, SepMode::NoTrailing, opt(spbr), tag("x"))(
                "x xxz"
            ),
            Ok(("z", vec!["x", "x", "x"]))
        );
    }

    #[test]
    fn sep_list1_no_trailing_test() {
        assert_eq!(
            sep_list::<_, _, VerboseError<_>, _, _>(1, SepMode::NoTrailing, opt(spbr), tag("x"))(
                "x z"
            ),
            Ok((" z", vec!["x"]))
        );
    }

    #[test]
    fn sep_list1_trailing_test() {
        assert_eq!(
            sep_list::<_, _, VerboseError<_>, _, _>(1, SepMode::AllowTrailing, opt(spbr), tag("x"))(
                "x z"
            ),
            Ok(("z", vec!["x"]))
        );
    }
}
