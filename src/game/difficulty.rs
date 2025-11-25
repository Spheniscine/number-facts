use super::Op;

#[derive(Clone, Copy, Debug)]
pub enum StringKind {
    Normal, Math
}

#[derive(Clone, Debug)]
pub struct Difficulty {
    pub op: Op,
    pub low: i32,
    pub high: i32,
    pub multiplier: i32,
    pub description: &'static [(StringKind, &'static str)]
}

pub const ADDITION_DIFFICULTIES: &[Difficulty] = &[
    Difficulty { op: Op::Plus, low: 0, high: 10, multiplier: 1, description: &[
        (StringKind::Normal, "Up to "),
        (StringKind::Math, "10",)
    ] },

    Difficulty { op: Op::Plus, low: 0, high: 20, multiplier: 1, description: &[
        (StringKind::Normal, "Up to "),
        (StringKind::Math, "20"),
    ] },

    Difficulty { op: Op::Plus, low: 0, high: 50, multiplier: 1, description: &[
        (StringKind::Normal, "Up to "),
        (StringKind::Math, "50"),
    ] },

    Difficulty { op: Op::Plus, low: 0, high: 100, multiplier: 1, description: &[
        (StringKind::Normal, "Up to "),
        (StringKind::Math, "100"),
    ] },

    Difficulty { op: Op::Plus, low: 0, high: 10, multiplier: 10, description: &[
        (StringKind::Normal, "Tens up to "),
        (StringKind::Math, "100"),
    ] },

    Difficulty { op: Op::Plus, low: 0, high: 10, multiplier: -1, description: &[
        (StringKind::Normal, "Negative to "),
        (StringKind::Math, "-10"),
    ] },
];

pub const MULTIPLICATION_DIFFICULTIES: &[Difficulty] = &[
    Difficulty { op: Op::Times, low: 2, high: 5, multiplier: 1, description: &[
        (StringKind::Math, "2"),
        (StringKind::Normal, " to "),
        (StringKind::Math, "5"),
        (StringKind::Normal, " Times Tables"),
    ] },

    Difficulty { op: Op::Times, low: 6, high: 12, multiplier: 1, description: &[
        (StringKind::Math, "6"),
        (StringKind::Normal, " to "),
        (StringKind::Math, "12"),
        (StringKind::Normal, " Times Tables"),
    ] },

    Difficulty { op: Op::Times, low: 2, high: 12, multiplier: 1, description: &[
        (StringKind::Math, "2"),
        (StringKind::Normal, " to "),
        (StringKind::Math, "12"),
        (StringKind::Normal, " Times Tables"),
    ] },

    Difficulty { op: Op::Times, low: 2, high: 12, multiplier: 10, description: &[
        (StringKind::Math, "2"),
        (StringKind::Normal, " to "),
        (StringKind::Math, "12"),
        (StringKind::Normal, " Multiples of "),
        (StringKind::Math, "10"),
    ] },
];