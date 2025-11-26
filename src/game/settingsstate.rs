use serde::{Deserialize, Serialize};

use super::{addition_difficulties, multiplication_difficulties, Difficulty, Op};

#[derive(Clone)]
pub struct SettingsState {
    pub difficulty_options: Difficulty,
    pub audio_state: i32,
    pub reset_level: bool,
}

impl SettingsState {
    pub fn op(&self) -> Op {
        self.difficulty_options.op
    }

    pub fn change_op(&mut self, op: Op) {
        if op == self.op() { return; }
        self.difficulty_options = match op {
            Op::Plus | Op::Minus => {
                addition_difficulties()[0].clone()
            }
            Op::Times | Op::Divide => {
                multiplication_difficulties()[0].clone()
            }
        }
    }

    pub fn change_difficulty(&mut self, index: usize) {
        self.difficulty_options = match self.op() {
            Op::Plus | Op::Minus => {
                addition_difficulties().get(index).unwrap_or_else(|| &addition_difficulties()[0]).clone()
            }
            Op::Times | Op::Divide => {
                multiplication_difficulties().get(index).unwrap_or_else(|| &multiplication_difficulties()[0]).clone()
            }
        }
    }
}