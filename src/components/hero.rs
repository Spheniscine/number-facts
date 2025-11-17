use dioxus::prelude::*;
use rand::{rng, Rng};
use strum::IntoEnumIterator;

use crate::{components::{operand::OperandValue, Advance, AudioPreloader, Check, FactComponent, Math, OpComponent, OpEntity, OpValue, OperandComponent, OperandEntity, Settings, Undo}, game::{self, Fact, GameState, Mark, Op, OptionMarkExt, ScreenState}};

#[component]
pub fn Hero() -> Element {
    // let rng = &mut rng();
    // let a = rng.random_range(0..10);
    // let b = rng.random_range(0..10);
    // let c = a + b;

    // let test_fact = Fact { operand1: Some(a), op: Some(Op::Plus), operand2: Some(b), result: Some(c), is_active: true };
    let mut game_state = use_signal(|| {
        GameState::new()
    });
    let state = game_state();
        
    rsx! {
        AudioPreloader {  }

        if game_state().screen_state == ScreenState::Settings {
            div {
                id: "hero",
                Settings {
                    game_state,
                }
            }
        } else {
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
                },

                div {
                    style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 1rem;",

                    if game_state.read().is_checked() {
                        Advance {
                            game_state,
                        },
                    } else {
                        Undo { 
                            game_state,
                        },

                        Check { 
                            game_state,
                        },
                    }
                },

                div {
                    id: "preloaded-images",

                    for mark in Mark::iter().map(|mark| Some(mark)).chain([None]) {
                        img {
                            src: mark.image_asset(),
                            width: 1,
                            height: 1,
                        }
                    }
                },
            }
        }
    }
}