#[derive(PartialEq, Eq, Debug)]
pub struct IdentifierExt<'s> {
    pub name: &'s str,
    pub type_args: Vec<IdentifierExt<'s>>,
    pub is_nullable: bool,
}

impl<'s> IdentifierExt<'s> {
    pub fn name(name: &'s str) -> Self {
        Self {
            name,
            type_args: Vec::default(),
            is_nullable: false,
        }
    }
}
