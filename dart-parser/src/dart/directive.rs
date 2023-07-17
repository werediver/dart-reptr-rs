#[derive(PartialEq, Eq, Debug)]
pub enum Directive<'s> {
    Export(&'s str),
    Import(Import<'s>),
    Part(&'s str),
    PartOf(PartOf<'s>),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Import<'s> {
    pub target: &'s str,
    pub prefix: Option<&'s str>,
    pub show: Vec<&'s str>,
    pub hide: Vec<&'s str>,
}

impl<'s> Import<'s> {
    pub fn target(target: &'s str) -> Self {
        Import {
            target,
            prefix: None,
            show: Vec::default(),
            hide: Vec::default(),
        }
    }

    pub fn target_as(target: &'s str, prefix: &'s str) -> Self {
        Import {
            target,
            prefix: Some(prefix),
            show: Vec::default(),
            hide: Vec::default(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum PartOf<'s> {
    LibPath(&'s str),
    /// Discouraged, but allowed.
    LibName(&'s str),
}
