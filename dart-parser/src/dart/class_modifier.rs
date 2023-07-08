/// The possible combinations are:
///
/// - `mixin` or `class`
/// - `mixin class`
/// - `base mixin` (only the `base` modifier can appear before a mixin declaration)
/// - `class` with other modifiers
///
/// See [Class modifiers](https://dart.dev/language/class-modifiers).
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum ClassModifier {
    Class = 1 << 1,
    Mixin = 1 << 2,
    Abstract = 1 << 3,
    Base = 1 << 4,
    Final = 1 << 5,
    Interface = 1 << 6,
    Sealed = 1 << 7,
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Debug)]
pub struct ClassModifierSet(usize);

impl ClassModifierSet {
    pub fn with(&self, item: ClassModifier) -> Self {
        Self(self.0 | item as usize)
    }

    pub fn contains(&self, item: ClassModifier) -> bool {
        self.0 & item as usize == item as usize
    }
}

impl FromIterator<ClassModifier> for ClassModifierSet {
    fn from_iter<T: IntoIterator<Item = ClassModifier>>(iter: T) -> Self {
        let mut set = ClassModifierSet::default();

        for item in iter {
            set = set.with(item);
        }

        set
    }
}
