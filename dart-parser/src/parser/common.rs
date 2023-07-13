use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while, take_while_m_n},
    character::complete::char,
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair},
    IResult,
};

/// Parse one or more whitespace characters, excluding line breaks.
pub fn sp(s: &str) -> IResult<&str, &str> {
    is_a(" \t")(s)
}

/// Parse one or more whitespace characters, including line breaks.
pub fn spbr(s: &str) -> IResult<&str, &str> {
    is_a(" \t\r\n")(s)
}

/// Parse exactly one line break.
pub fn br(s: &str) -> IResult<&str, &str> {
    alt((tag("\n"), tag("\r\n"), tag("\r")))(s)
}

pub fn identifier(s: &str) -> IResult<&str, &str> {
    fn is_start_char(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || c == '$'
    }

    fn is_part_char(c: char) -> bool {
        is_start_char(c) || c.is_ascii_digit()
    }

    recognize(pair(
        take_while_m_n(1, 1, is_start_char),
        take_while(is_part_char),
    ))(s)
}

pub fn block(s: &str) -> IResult<&str, &str> {
    recognize(delimited(
        char('{'),
        recognize(many0(alt((is_not("{}"), block)))),
        char('}'),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sp_test() {
        let s = "  \n\t\r\nx";
        assert_eq!(spbr(s), Ok(("x", "  \n\t\r\n")));
    }
}
