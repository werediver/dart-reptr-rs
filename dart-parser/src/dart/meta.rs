use super::{Annotation, Comment};

#[derive(PartialEq, Eq, Debug)]
pub struct WithMeta<'s, T> {
    pub meta: Vec<Meta<'s>>,
    value: T,
}

impl<'s, T> WithMeta<'s, T> {
    pub fn new(meta: Vec<Meta<'s>>, value: T) -> Self {
        Self { meta, value }
    }

    pub fn value(value: T) -> Self {
        Self {
            meta: Vec::new(),
            value,
        }
    }
}

impl<'s, T> AsRef<T> for WithMeta<'s, T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Meta<'s> {
    Annotation(Annotation<'s>),
    Comment(Comment<'s>),
}
