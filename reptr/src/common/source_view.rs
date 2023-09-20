#[derive(Debug)]
pub struct SourceView<T> {
    source: String,
    view: T,
}

impl<T> SourceView<T> {
    pub fn try_new<'s, F, E>(source: String, f: F) -> Result<Self, E>
    where
        T: 's,
        F: FnOnce(&'s str) -> Result<T, E>,
    {
        match f(unsafe { erase_lifetime(&source) }) {
            Ok(view) => Ok(Self { source, view }),
            Err(e) => Err(e),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

impl<T> AsRef<T> for SourceView<T> {
    fn as_ref(&self) -> &T {
        &self.view
    }
}

unsafe fn erase_lifetime<'a, T: ?Sized>(value: &T) -> &'a T {
    &*(value as *const _)
}
