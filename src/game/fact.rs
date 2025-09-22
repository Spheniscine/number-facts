use super::Op;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fact {
    pub operand1: Option<i32>,
    pub op: Option<Op>,
    pub operand2: Option<i32>,
    pub result: Option<i32>,
    pub is_active: bool,
}