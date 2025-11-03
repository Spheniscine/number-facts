use rand::{rng, seq::SliceRandom};

use super::{Audio, Fact, Feedback, FeedbackImpl, Mark, Op};

#[derive(Clone, Debug)]
pub struct GameState {
    pub facts: [Fact; 4],
    pub solution: [Fact; 4],
    pub operands: [i32; 3],
    pub ops: [Op; 2],
    pub marks: Option<[Mark; 4]>,
    pub feedback: FeedbackImpl,
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

        let mut facts = [Fact::default(); 4];
        facts[0].is_active = true;

        let mut operands = [a, b, c];
        let mut ops = [Op::Plus, Op::Minus];

        solution.sort();
        operands.shuffle(rng);
        ops.shuffle(rng);

        GameState { facts, solution, operands, ops, marks: None, feedback: FeedbackImpl { audio_state: 1., prev_audio_state: 1. } }
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
        // todo
    }
}