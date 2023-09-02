use nom::{
    branch::alt,
    combinator::opt,
    error::{context, ContextError, ParseError},
    multi::many0,
    sequence::terminated,
    Parser,
};

use crate::dart::{meta::Meta, WithMeta};

use super::{annotation, comment, common::spbr, PResult};

pub fn with_meta<'s, P, T, E>(mut p: P) -> impl FnMut(&'s str) -> PResult<WithMeta<'s, T>, E>
where
    P: Parser<&'s str, T, E>,
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context("with_meta", move |s| {
        let (s, meta) = meta(s)?;
        let (s, value) = p.parse(s)?;

        Ok((s, WithMeta::new(meta, value)))
    })
}

fn meta<'s, E>(s: &'s str) -> PResult<Vec<Meta>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context("meta", many0(terminated(meta_item, opt(spbr))))(s)
}

fn meta_item<'s, E>(s: &'s str) -> PResult<Meta, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((comment.map(Meta::Comment), annotation.map(Meta::Annotation)))(s)
}
