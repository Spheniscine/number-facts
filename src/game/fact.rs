use super::Op;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Fact {
    pub operand1: Option<i32>,
    pub op: Option<Op>,
    pub operand2: Option<i32>,
    pub result: Option<i32>,
    pub is_active: bool,
}

impl Default for Fact {
    fn default() -> Self {
        Self { operand1: None, op: None, operand2: None, result: None, is_active: false }
    }
}

impl Fact {
    pub fn is_complete(&self) -> bool {
        self.operand1.is_some() && self.op.is_some() && self.operand2.is_some() && self.result.is_some()
    }
}