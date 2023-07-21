use tiny_set::with_tiny_set;

#[with_tiny_set]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(usize)]
pub enum MemberModifier {
    External,
    Static,
    Const,
    Final,
    Late,
    Covariant,
}
