use std::{rc::Rc, slice, str};

use memmap2::Mmap;

/// Source is supposed to be consumed as [`AsRef<str>`].
///
/// Source can be constructed from [`String`] via [`From<String>::from`]
/// or from [`Mmap`] via [`TryFrom<Mmap>::try_from`].
#[derive(Clone, Debug)]
pub struct Source {
    inner: SourceInner,
}

#[derive(Clone, Debug)]
enum SourceInner {
    String(String),
    Mmap(Rc<Mmap>, *const u8, usize),
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        match &self.inner {
            SourceInner::String(s) => s,
            SourceInner::Mmap(_, s_ptr, s_len) => unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(*s_ptr, *s_len))
            },
        }
    }
}

impl From<String> for Source {
    fn from(value: String) -> Self {
        Self {
            inner: SourceInner::String(value),
        }
    }
}

impl TryFrom<Mmap> for Source {
    type Error = str::Utf8Error;

    fn try_from(value: Mmap) -> Result<Self, Self::Error> {
        let s = str::from_utf8(&value)?;
        let (s_ptr, s_len) = (s.as_ptr(), s.len());

        Ok(Self {
            inner: SourceInner::Mmap(Rc::new(value), s_ptr, s_len),
        })
    }
}
