use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{addition_difficulties, multiplication_difficulties, Difficulty, Op};

pub const NAME_OP: &str = "op";
pub const VALUE_OP_PLUS: &str = "plus";
pub const VALUE_OP_TIMES: &str = "times";
pub const NAME_DIFFICULTY_CHOICE: &str = "difficulty_choice";

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

    pub fn change_difficulty(&mut self, index: usize) -> bool {
        self.difficulty_options = match self.op() {
            Op::Plus | Op::Minus => {
                if index < addition_difficulties().len() {
                    addition_difficulties()[index].clone()
                } else { return false; }
            }
            Op::Times | Op::Divide => {
                if index < multiplication_difficulties().len() {
                    multiplication_difficulties()[index].clone()
                } else { return false; }
            }
        };
        true
    }

    pub fn parse_radio_button_change(&mut self, name: &str, value: &str) -> bool {
        match name {
            NAME_OP => {
                let op = match value {
                    VALUE_OP_PLUS => Op::Plus,
                    VALUE_OP_TIMES => Op::Times,
                    _ => {return false;}
                };
                self.change_op(op);
            }
            NAME_DIFFICULTY_CHOICE => {
                let Ok(index) = usize::from_str(value) else {return false};
                return self.change_difficulty(index);
            }
            _ => {return false;}
        }
        true
    }
}