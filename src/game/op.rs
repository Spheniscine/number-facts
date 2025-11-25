use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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