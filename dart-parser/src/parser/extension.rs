use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{cut, opt},
    error::{context, ContextError, ParseError},
    multi::many0,
    sequence::{pair, preceded, terminated, tuple},
    Parser,
};

use crate::dart::{extension::ExtensionMember, Extension};

use super::{
    annotation::annotation,
    comment::comment,
    common::spbr,
    func_like::func_like,
    identifier::{identifier, identifier_ext},
    type_params::type_params,
    PResult,
};

pub fn extension<'s, E>(s: &'s str) -> PResult<Extension, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "extension",
        tuple((
            terminated(tag("extension"), spbr),
            opt(terminated(identifier, opt(spbr))),
            opt(terminated(type_params, opt(spbr))),
            terminated(tag("on"), spbr),
            terminated(identifier_ext, opt(spbr)),
            extension_body,
        ))
        .map(|(_, name, type_params, _, on, body)| Extension {
            name,
            type_params: type_params.unwrap_or(Vec::new()),
            on,
            body,
        }),
    )(s)
}

pub fn extension_body<'s, E>(s: &'s str) -> PResult<Vec<ExtensionMember>, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "extension_body",
        preceded(
            pair(tag("{"), opt(spbr)),
            cut(terminated(
                many0(terminated(extension_member, opt(spbr))),
                tag("}"),
            )),
        ),
    )(s)
}

fn extension_member<'s, E>(s: &'s str) -> PResult<ExtensionMember, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    alt((
        comment.map(ExtensionMember::Comment),
        annotation.map(ExtensionMember::Annotation),
        func_like.map(ExtensionMember::FuncLike),
    ))(s)
}
