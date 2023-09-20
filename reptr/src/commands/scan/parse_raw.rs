use std::{fmt::Display, string::FromUtf8Error, sync::Arc};

use dart_parser::{Dart, WithMeta};

use crate::common::SourceView;

use super::{stats, time};

pub async fn parse_raw_async<Counter>(
    source: Vec<u8>,
    stats: Arc<std::sync::Mutex<Counter>>,
) -> Result<SourceView<Vec<WithMeta<'static, Dart<'static>>>>, ParseRawError>
where
    Counter: stats::Counter<stats::event::FileParsed> + Send + 'static,
{
    let (ch_sink, ch_source) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let ast = parse_raw(source, stats);
        ch_sink.send(ast).unwrap();
    });

    ch_source.await.unwrap()
}

pub fn parse_raw<Counter>(
    source: Vec<u8>,
    stats: Arc<std::sync::Mutex<Counter>>,
) -> Result<SourceView<Vec<WithMeta<'static, Dart<'static>>>>, ParseRawError>
where
    Counter: stats::Counter<stats::event::FileParsed>,
{
    let (source, utf8_validation_duration) = time! { String::from_utf8(source)? };

    let (ast, parsing_duration) = time! { SourceView::try_new(source, dart_parser::parse)? };

    let event = stats::event::FileParsed {
        size: ast.source().len(),
        utf8_validation_duration,
        parsing_duration,
    };
    stats.lock()?.count(event);

    Ok(ast)
}

#[derive(Debug)]
pub enum ParseRawError {
    SyncPoisoned,
    InvalidUtf8(FromUtf8Error),
    ParseError(String),
}

impl Display for ParseRawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRawError::SyncPoisoned => f.write_str("A synchronization primitive is poisoned"),
            ParseRawError::InvalidUtf8(e) => e.fmt(f),
            ParseRawError::ParseError(e) => e.fmt(f),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for ParseRawError {
    fn from(_value: std::sync::PoisonError<T>) -> Self {
        ParseRawError::SyncPoisoned
    }
}

impl From<FromUtf8Error> for ParseRawError {
    fn from(value: FromUtf8Error) -> Self {
        ParseRawError::InvalidUtf8(value)
    }
}

impl From<String> for ParseRawError {
    fn from(value: String) -> Self {
        ParseRawError::ParseError(value)
    }
}
