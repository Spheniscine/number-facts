use dioxus::logger::tracing::info;
use rand::{rng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::{addition_difficulties, difficulty, Audio, Difficulty, Fact, Feedback, FeedbackImpl, Mark, Op, SettingsState};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScreenState {
    Game, Settings
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub facts: [Fact; 4],
    pub solution: [Fact; 4],
    pub operands: [i32; 3],
    pub ops: [Op; 2],
    pub marks: Option<[Mark; 4]>,
    pub feedback: FeedbackImpl,
    pub screen_state: ScreenState,
    pub difficulty: Difficulty,
    pub settings_cancelable: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut solution = [Fact::default(); 4];

        let mut facts = [Fact::default(); 4];

        let mut operands = [0, 0, 0];
        let mut ops = [Op::Plus, Op::Minus];

        let mut res = GameState { facts, solution, operands, ops, marks: None, feedback: FeedbackImpl { audio_state: 1., prev_audio_state: 1. },
            difficulty: addition_difficulties()[0].clone(),
            screen_state: ScreenState::Settings, settings_cancelable: false };
        res.generate_test();
        res
    }

    pub fn generate(&mut self, difficulty: Difficulty) {
        let rng = &mut rng();
        let (a, b, c, mut solution, mut ops) = match difficulty.op {
            Op::Plus | Op::Minus => {
                let low = difficulty.low;
                let high = difficulty.high;
                let mult = difficulty.multiplier;
        
                let [a, b, c] = loop {
                    let mut x = rng.random_range(low..=high) * mult;
                    let mut y = rng.random_range(low..high) * mult;
                    if y >= x { y += mult; }
                    if y < x { std::mem::swap(&mut x, &mut y); }
                    if y - x == x { continue; }

                    let ans = [x, y-x, y];

                    // if mult is negative, ensure one of the values is negative
                    if mult >= 0 || ans.iter().any(|&x| x < 0) {
                        break ans;
                    }
                };

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
                (a, b, c, solution, [Op::Plus, Op::Minus])
            },
            Op::Times | Op::Divide => {
                let low = difficulty.low;
                let high = difficulty.high;
                let mult = difficulty.multiplier;
        
                let a = rng.random_range(low..=high) * mult;
                let b = loop {
                    let b = rng.random_range(1..=12);
                    if b != a { break b; }
                };
                let c = a * b;

                let mut solution = [
                    Fact {
                        operand1: Some(a),
                        op: Some(Op::Times),
                        operand2: Some(b),
                        result: Some(c),
                        is_active: false,
                    },
                    Fact {
                        operand1: Some(b),
                        op: Some(Op::Times),
                        operand2: Some(a),
                        result: Some(c),
                        is_active: false,
                    },
                    Fact {
                        operand1: Some(c),
                        op: Some(Op::Divide),
                        operand2: Some(a),
                        result: Some(b),
                        is_active: false,
                    },
                    Fact {
                        operand1: Some(c),
                        op: Some(Op::Divide),
                        operand2: Some(b),
                        result: Some(a),
                        is_active: false,
                    },
                ];
                (a, b, c, solution, [Op::Times, Op::Divide])
            },
        };

        let mut facts = [Fact::default(); 4];
        facts[0].is_active = true;

        let mut operands = [a, b, c];

        solution.sort();
        operands.shuffle(rng);
        ops.shuffle(rng);

        self.facts = facts;
        self.solution = solution;
        self.operands = operands;
        self.ops = ops;
        self.marks = None;
        self.difficulty = difficulty;
        self.settings_cancelable = true;
    }

    fn generate_test(&mut self) {
        let limit = 10;
        let rng = &mut rng();
        let (a, b, c) = loop {
            let mut x = rng.random_range(0..=limit);
            let mut y = rng.random_range(0..limit);
            if y >= x { y += 1; }
            if y < x { std::mem::swap(&mut x, &mut y); }
            if y - x == x { continue; }

            break (x, y-x, y);
        };

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

        let mut facts = [Fact::default(); 4];
        facts[0].is_active = true;

        let mut operands = [a, b, c];
        let mut ops = [Op::Plus, Op::Minus];

        solution.sort();
        operands.shuffle(rng);
        ops.shuffle(rng);

        self.facts = facts;
        self.solution = solution;
        self.operands = operands;
        self.ops = ops;
        self.marks = None;
    }

    fn update_active(&mut self) {
        let x = (0..self.facts.len()).find(|&i| {
            !self.facts[i].is_complete()
        });
        for i in 0..self.facts.len() {
            self.facts[i].is_active = x == Some(i);
        }
    }

    pub fn click_operand(&mut self, value: i32) -> bool {
        let mut ok = false;
        for i in 0..self.facts.len() {
            if self.facts[i].operand1.is_none() {
                self.facts[i].operand1 = Some(value);
                ok = true;
                break;
            }
            if self.facts[i].op.is_none() {
                return false;
            }
            if self.facts[i].operand2.is_none() {
                self.facts[i].operand2 = Some(value);
                ok = true;
                break;
            }
            if self.facts[i].result.is_none() {
                self.facts[i].result = Some(value);
                ok = true;
                break;
            }
        }

        if ok { self.update_active(); }
        ok
    }

    pub fn click_op(&mut self, value: Op) -> bool {
        let mut ok = false;
        for i in 0..self.facts.len() {
            if self.facts[i].operand1.is_none() {
                return false;
            }
            if self.facts[i].op.is_none() {
                self.facts[i].op = Some(value);
                ok = true;
                break;
            }
            if self.facts[i].operand2.is_none() {
                return false;
            }
            if self.facts[i].result.is_none() {
                return false;
            }
        }

        if ok { self.update_active(); }
        ok
    }

    pub fn undo(&mut self) -> bool {
        let mut ok = false;
        for i in (0..self.facts.len()).rev() {
            if self.facts[i].result.is_some() {
                self.facts[i].result = None;
                ok = true;
                break;
            }
            if self.facts[i].operand2.is_some() {
                self.facts[i].operand2 = None;
                ok = true;
                break;
            }
            if self.facts[i].op.is_some() {
                self.facts[i].op = None;
                ok = true;
                break;
            }
            if self.facts[i].operand1.is_some() {
                self.facts[i].operand1 = None;
                ok = true;
                break;
            }
        }

        if ok { self.update_active(); }
        ok
    }

    pub fn is_complete(&self) -> bool {
        self.facts.iter().all(|fact| fact.is_complete())
    }

    pub fn check(&mut self) {
        if !self.is_complete() { return; }
        let mut used = [false; 4];
        let mut marks = [Mark::Correct; 4];
        for i in 0..4 {
            let j = (0..4).find(|&j| !used[j] && self.facts[i] == self.solution[j]);
            if let Some(j) = j {
                used[j] = true;
                marks[i] = Mark::Correct;
            } else {
                let rep = (0..4).any(|j| self.facts[i] == self.solution[j]);
                marks[i] = if rep {Mark::Repeat} else {Mark::Wrong};
            }
        }
        self.marks = Some(marks);
        if self.is_correct() {
            self.feedback.play_audio(Audio::Correct);
        } else {
            self.feedback.play_audio(Audio::Wrong);
        }
    }

    pub fn is_checked(&self) -> bool {
        self.marks.is_some()
    }

    pub fn is_correct(&self) -> bool {
        self.marks == Some([Mark::Correct; 4])
    }

    pub fn advance(&mut self) {
        if !self.is_complete() { return; }
        
        if self.is_correct() {
            self.generate(self.difficulty.clone());
        } else {
            let mut facts = [Fact::default(); 4];

            for (i, f) in (0..4).filter(|&i| self.marks.map(|m| m[i]) == Some(Mark::Correct)).zip(&mut facts) {
                *f = self.facts[i]
            }

            self.facts = facts;
            self.marks = None;
            self.update_active();
        }
    }

    pub fn get_settings_state(&self) -> SettingsState {
        SettingsState {
            difficulty_options: self.difficulty.clone(),
            audio_state: (self.feedback.get_audio_state() * 100.).round() as i32,
            reset_level: !self.settings_cancelable,
        }
    }

    pub fn apply_settings(&mut self, settings: SettingsState) {
        self.feedback.set_audio_state(settings.audio_state as f64 / 100.);

        if self.difficulty != settings.difficulty_options || settings.reset_level {
            self.generate(settings.difficulty_options);
        }
    }

    pub fn toggle_audio(&mut self) {
        self.feedback.toggle_audio();
        // LocalStorage.save_game_state(&self);
    }
}