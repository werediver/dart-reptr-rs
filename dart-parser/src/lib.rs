mod dart;
mod parser;

pub use dart::Dart;

pub fn parse(s: &str) -> Result<Vec<Dart>, nom::Err<nom::error::Error<&str>>> {
    parser::parse(s).map(|(_, value)| value)
}
