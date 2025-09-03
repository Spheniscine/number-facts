
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Plus, Minus, Times, Divide
}

impl Op {
    pub fn to_tex(&self) -> &str {
        match self {
            Op::Plus => "+",
            Op::Minus => "-",
            Op::Times => "\\times",
            Op::Divide => "\\div",
        }
    }
}