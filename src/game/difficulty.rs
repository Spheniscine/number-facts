use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use super::Op;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringKind {
    Normal, Math
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Difficulty {
    pub op: Op,
    pub low: i32,
    pub high: i32,
    pub multiplier: i32, // if negative, means at least one of the operands must be negative
    pub description: Vec<(StringKind, String)>,
}

pub fn addition_difficulties() -> &'static [Difficulty] {
    static MEM: OnceLock<Vec<Difficulty>> = OnceLock::new();
    MEM.get_or_init(|| {
        vec![
            Difficulty { op: Op::Plus, low: 0, high: 10, multiplier: 1, description: vec![
                (StringKind::Normal, "Up to ".to_string()),
                (StringKind::Math, "10".to_string(),)
            ] },

            Difficulty { op: Op::Plus, low: 0, high: 20, multiplier: 1, description: vec![
                (StringKind::Normal, "Up to ".to_string()),
                (StringKind::Math, "20".to_string()),
            ] },

            Difficulty { op: Op::Plus, low: 0, high: 50, multiplier: 1, description: vec![
                (StringKind::Normal, "Up to ".to_string()),
                (StringKind::Math, "50".to_string()),
            ] },

            Difficulty { op: Op::Plus, low: 0, high: 100, multiplier: 1, description: vec![
                (StringKind::Normal, "Up to ".to_string()),
                (StringKind::Math, "100".to_string()),
            ] },

            Difficulty { op: Op::Plus, low: 0, high: 10, multiplier: 10, description: vec![
                (StringKind::Normal, "Tens up to ".to_string()),
                (StringKind::Math, "100".to_string()),
            ] },

            Difficulty { op: Op::Plus, low: -10, high: 10, multiplier: -1, description: vec![
                (StringKind::Normal, "Negative to ".to_string()),
                (StringKind::Math, "-10".to_string()),
            ] },
        ]
    })
}

pub fn multiplication_difficulties() -> &'static [Difficulty] {
    static MEM: OnceLock<Vec<Difficulty>> = OnceLock::new();
    MEM.get_or_init(|| {
        vec![
            Difficulty { op: Op::Times, low: 2, high: 5, multiplier: 1, description: vec![
                (StringKind::Math, "2".to_string()),
                (StringKind::Normal, " to ".to_string()),
                (StringKind::Math, "5".to_string()),
                (StringKind::Normal, " Times Tables".to_string()),
            ] },

            Difficulty { op: Op::Times, low: 6, high: 12, multiplier: 1, description: vec![
                (StringKind::Math, "6".to_string()),
                (StringKind::Normal, " to ".to_string()),
                (StringKind::Math, "12".to_string()),
                (StringKind::Normal, " Times Tables".to_string()),
            ] },

            Difficulty { op: Op::Times, low: 2, high: 12, multiplier: 1, description: vec![
                (StringKind::Math, "2".to_string()),
                (StringKind::Normal, " to ".to_string()),
                (StringKind::Math, "12".to_string()),
                (StringKind::Normal, " Times Tables".to_string()),
            ] },

            Difficulty { op: Op::Times, low: 2, high: 12, multiplier: 10, description: vec![
                (StringKind::Math, "2".to_string()),
                (StringKind::Normal, " to ".to_string()),
                (StringKind::Math, "12".to_string()),
                (StringKind::Normal, " Multiples of ".to_string()),
                (StringKind::Math, "10".to_string()),
            ] },
        ]
    })
}