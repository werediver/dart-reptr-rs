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
    ty::{identifier, ty},
    type_params::type_params,
    var, PResult,
};

pub fn extension<'s, E>(s: &'s str) -> PResult<Extension, E>
where
    E: ParseError<&'s str> + ContextError<&'s str>,
{
    context(
        "extension",
        tuple((
            alt((
                tuple((tag("extension"), spbr, tag("on"), spbr)).map(|_| (None, Vec::new())),
                tuple((
                    terminated(tag("extension"), spbr),
                    terminated(identifier, opt(spbr)),
                    opt(terminated(type_params, opt(spbr))),
                    terminated(tag("on"), spbr),
                ))
                .map(|(_, name, type_params, _)| (Some(name), type_params.unwrap_or(Vec::new()))),
            )),
            terminated(ty, opt(spbr)),
            extension_body,
        ))
        .map(|((name, type_params), on, body)| Extension {
            name,
            type_params,
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
        var.map(ExtensionMember::Var),
    ))(s)
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::dart::{ty::Type, NotFuncType};

    use super::*;

    #[test]
    fn extension_named() {
        assert_eq!(
            extension::<VerboseError<_>>("extension X on Y {}x"),
            Ok((
                "x",
                Extension {
                    name: Some("X"),
                    type_params: Vec::new(),
                    on: Type::NotFunc(NotFuncType {
                        name: "Y",
                        type_args: Vec::new(),
                        is_nullable: false
                    }),
                    body: Vec::new(),
                }
            ))
        );
    }

    #[test]
    fn extension_unnamed() {
        assert_eq!(
            extension::<VerboseError<_>>("extension on Y {}x"),
            Ok((
                "x",
                Extension {
                    name: None,
                    type_params: Vec::new(),
                    on: Type::NotFunc(NotFuncType {
                        name: "Y",
                        type_args: Vec::new(),
                        is_nullable: false
                    }),
                    body: Vec::new(),
                }
            ))
        );
    }
}
