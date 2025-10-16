use dioxus::prelude::*;
use rand::{rng, Rng};

use crate::{components::{operand::OperandValue, Check, FactComponent, Math, OpComponent, OpEntity, OpValue, OperandComponent, OperandEntity, Undo}, game::{self, Fact, GameState, Op}};

#[component]
pub fn Hero() -> Element {
    // let rng = &mut rng();
    // let a = rng.random_range(0..10);
    // let b = rng.random_range(0..10);
    // let c = a + b;

    // let test_fact = Fact { operand1: Some(a), op: Some(Op::Plus), operand2: Some(b), result: Some(c), is_active: true };
    let mut game_state = use_signal(|| {
        GameState::new_test()
    });
    let state = game_state();
        
    rsx! {
        div {
            id: "hero",

            p {
                style: "font-size:5rem; color:#fff; padding: 3rem;",
                "Find the number fact family using the buttons on the bottom"
            },

            for i in 0..state.facts.len() {
                FactComponent { 
                    fact: state.facts[i].clone(),
                    mark: state.marks.map(|marks| marks[i]),
                }
            }

            div {
                style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 2rem;",

                for operand in state.operands {
                    OperandEntity { 
                        value: operand,
                        game_state,
                    },
                }

                for op in state.ops {
                    OpEntity { 
                        value: op,
                        game_state,
                    },
                }
            }

            div {
                style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 1rem;",
                
                Undo { 
                    game_state,
                },

                Check { 
                    game_state,
                },
            }
        }
    }
}