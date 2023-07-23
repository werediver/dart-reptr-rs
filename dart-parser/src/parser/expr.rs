use nom::combinator::recognize;

use super::{string::string_simple, PResult};

pub fn expr(s: &str) -> PResult<&str> {
    // This is a stub implementation
    recognize(string_simple)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr_string() {
        assert_eq!(expr("\"text\" "), Ok((" ", "\"text\"")));
    }
}
