#[derive(PartialEq, Eq, Debug)]
pub enum Directive<'s> {
    Export(Export<'s>),
    Import(Import<'s>),
    Part(&'s str),
    PartOf(PartOf<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Export<'s> {
    pub target: &'s str,
    pub filters: Vec<Filter<'s>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Import<'s> {
    pub target: &'s str,
    pub prefix: Option<&'s str>,
    pub filters: Vec<Filter<'s>>,
}

impl<'s> Import<'s> {
    pub fn target(target: &'s str) -> Self {
        Import {
            target,
            prefix: None,
            filters: Vec::default(),
        }
    }

    pub fn target_as(target: &'s str, prefix: &'s str) -> Self {
        Import {
            target,
            prefix: Some(prefix),
            filters: Vec::default(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Filter<'s> {
    Show(Vec<&'s str>),
    Hide(Vec<&'s str>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum PartOf<'s> {
    LibPath(&'s str),
    /// Discouraged, but allowed.
    LibName(&'s str),
}
