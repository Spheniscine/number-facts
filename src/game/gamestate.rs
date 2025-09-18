use super::{Fact, Op};

pub struct GameState {
    pub facts: [Fact; 4],
    pub solution: [Fact; 4],
    pub operands: [i32; 3],
    pub ops: [Op; 2],
}