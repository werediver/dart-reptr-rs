use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    combinator::{cut, recognize},
    multi::many0_count,
    sequence::{preceded, terminated},
    IResult,
};

pub fn string_simple(s: &str) -> IResult<&str, &str> {
    let dq = preceded(
        tag("\""),
        cut(terminated(
            many0_count(alt((tag("\\\""), is_not("\"\r\n")))),
            tag("\""),
        )),
    );
    let sq = preceded(
        tag("'"),
        cut(terminated(
            many0_count(alt((tag("\\'"), is_not("'\r\n")))),
            tag("'"),
        )),
    );

    recognize(alt((dq, sq)))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_simple_test() {
        let sq = r#""as\"${df}'gh'\"x"#;
        assert_eq!(string_simple(sq), Ok(("x", sq)));

        let dq = r#"'as\'${df}"gh"\'x"#;
        assert_eq!(string_simple(dq), Ok(("x", dq)));
    }
}
