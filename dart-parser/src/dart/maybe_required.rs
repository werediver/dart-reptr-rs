use std::fmt::Debug;

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

impl<T> PartialEq<MaybeRequired<T>> for MaybeRequired<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &MaybeRequired<T>) -> bool {
        self.is_required == other.is_required && self.value == other.value
    }
}

impl<T> Eq for MaybeRequired<T> where MaybeRequired<T>: PartialEq {}

impl<T> Debug for MaybeRequired<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaybeRequired")
            .field("is_required", &self.is_required)
            .field("value", &self.value)
            .finish()
    }
}
