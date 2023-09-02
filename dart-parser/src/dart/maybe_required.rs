use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug)]
pub struct MaybeRequired<T> {
    pub is_required: bool,
    value: T,
}

impl<T> MaybeRequired<T> {
    pub fn new(is_required: bool, value: T) -> Self {
        Self { is_required, value }
    }

    pub fn required(value: T) -> Self {
        Self {
            is_required: true,
            value,
        }
    }

    pub fn optional(value: T) -> Self {
        Self {
            is_required: false,
            value,
        }
    }
}

impl<T> AsRef<T> for MaybeRequired<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}
