use rand::{rng, seq::SliceRandom};

use super::{Fact, Op};

pub struct GameState {
    pub facts: [Fact; 4],
    pub solution: [Fact; 4],
    pub operands: [i32; 3],
    pub ops: [Op; 2],
}

impl GameState {
    pub fn new_test() -> Self {
        let rng = &mut rng();

        let a = 1;
        let b = 2;
        let c = a + b;

        let mut solution = [
            Fact {
                operand1: Some(a),
                op: Some(Op::Plus),
                operand2: Some(b),
                result: Some(c),
                is_active: false,
            },
            Fact {
                operand1: Some(b),
                op: Some(Op::Plus),
                operand2: Some(a),
                result: Some(c),
                is_active: false,
            },
            Fact {
                operand1: Some(c),
                op: Some(Op::Minus),
                operand2: Some(a),
                result: Some(b),
                is_active: false,
            },
            Fact {
                operand1: Some(c),
                op: Some(Op::Minus),
                operand2: Some(b),
                result: Some(a),
                is_active: false,
            },
        ];

        let mut operands = [a, b, c];
        let mut ops = [Op::Plus, Op::Minus];

        solution.sort();
        operands.shuffle(rng);
        ops.shuffle(rng);

        GameState { facts: solution.clone(), solution, operands, ops }
    }
}