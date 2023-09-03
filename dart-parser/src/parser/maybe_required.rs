use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{opt, success},
    error::{context, ContextError, ParseError},
    sequence::terminated,
    Parser,
};

use crate::dart::MaybeRequired;

use super::{common::spbrc, PResult};

pub fn maybe_required<'s, P, T, E>(mut p: P) -> impl FnMut(&'s str) -> PResult<MaybeRequired<T>, E>
where
    P: Parser<&'s str, T, E>,
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context("maybe_required", move |s| {
        let (s, is_required) = terminated(is_required, opt(spbrc))(s)?;
        let (s, value) = p.parse(s)?;

        Ok((s, MaybeRequired::new(is_required, value)))
    })
}

fn is_required<'s, E>(s: &'s str) -> PResult<bool, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((tag("required").map(|_| true), success(false)))(s)
}
