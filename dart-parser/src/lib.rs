mod dart;
mod parser;

pub use dart::Dart;
use dart::WithMeta;
use nom::{
    error::{convert_error, VerboseError},
    Err,
};

pub fn parse(s: &impl AsRef<str>) -> Result<Vec<WithMeta<Dart>>, String> {
    let s = s.as_ref();

    // Using the simple `nom::error::Error` may be more efficient,
    // but `nom::error::VerboseError` allows for much better error messages,
    // which is advantageous for development and debugging.
    parser::parse::<VerboseError<_>>(s)
        .map(|(_, value)| value)
        .map_err(|err| match err {
            Err::Incomplete(_) => "Incomplete input".to_owned(),
            Err::Error(err) | Err::Failure(err) => convert_error(s, err),
        })
}
