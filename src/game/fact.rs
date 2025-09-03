use super::Op;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Fact {
    operand1: Option<i32>,
    op: Option<Op>,
    operand2: Option<i32>,
    result: Option<i32>,
    is_active: bool,
}