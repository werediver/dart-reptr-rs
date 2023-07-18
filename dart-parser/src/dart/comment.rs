#[derive(PartialEq, Eq, Debug)]
pub enum Comment<'s> {
    SingleLine(&'s str),
    MultiLine(&'s str),
}
    