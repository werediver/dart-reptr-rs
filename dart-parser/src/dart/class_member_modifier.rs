#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum ClassMemberModifier {
    External = 1 << 1,
    Static = 1 << 2,
    Const = 1 << 3,
    Final = 1 << 4,
    Late = 1 << 5,
    Covariant = 1 << 6,
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Debug)]
pub struct ClassMemberModifierSet(usize);

impl ClassMemberModifierSet {
    pub fn with(&self, item: ClassMemberModifier) -> Self {
        Self(self.0 | item as usize)
    }

    pub fn contains(&self, item: ClassMemberModifier) -> bool {
        self.0 & item as usize == item as usize
    }
}

impl FromIterator<ClassMemberModifier> for ClassMemberModifierSet {
    fn from_iter<T: IntoIterator<Item = ClassMemberModifier>>(iter: T) -> Self {
        let mut set = ClassMemberModifierSet::default();

        for item in iter {
            set = set.with(item);
        }

        set
    }
}
