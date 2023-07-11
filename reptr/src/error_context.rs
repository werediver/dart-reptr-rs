use std::io;

pub trait ErrorContext {
    fn context_lazy<F, S>(self, mk_context: F) -> Self
    where
        F: FnOnce() -> S,
        S: AsRef<str>;

    fn context(self, context: impl AsRef<str>) -> Self;
}

impl<T> ErrorContext for io::Result<T> {
    fn context_lazy<F, S>(self, mk_context: F) -> io::Result<T>
    where
        F: FnOnce() -> S,
        S: AsRef<str>,
    {
        self.map_err(|err| {
            io::Error::new(err.kind(), format!("{}\n\n{err}", mk_context().as_ref()))
        })
    }

    fn context(self, context: impl AsRef<str>) -> Self {
        self.map_err(|err| io::Error::new(err.kind(), format!("{}\n\n{err}", context.as_ref())))
    }
}
