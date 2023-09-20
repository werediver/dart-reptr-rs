use std::io;

mod error_context;
mod read_dir_ext;
mod source_view;

pub use error_context::ErrorContext;
pub use read_dir_ext::{MapDirResult, ReadDirExt};
pub use source_view::SourceView;

/// Map [`std::sync::PoisonError`] to [`io::Error`].
pub fn poisoned<T>(_: std::sync::PoisonError<T>) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        "A synchronization primitive is poisoned",
    )
}
