use dioxus::prelude::*;
use rand::{rng, Rng};

use crate::{components::{operand::OperandValue, FactComponent, Math, OpComponent, OpValue, OperandComponent}, game::{Fact, GameState, Op}};

#[component]
pub fn Hero() -> Element {
    let rng = &mut rng();
    let a = rng.random_range(0..10);
    let b = rng.random_range(0..10);
    let c = a + b;

    let test_fact = Fact { operand1: Some(a), op: Some(Op::Plus), operand2: Some(b), result: Some(c), is_active: true };
    let state = GameState::new_test();
        
    rsx! {
        // div {
        //     id: "hero",
        //     img { src: HEADER_SVG, id: "header" }
        //     div { id: "links",
        //         a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
        //         a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
        //         a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
        //         a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
        //         a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
        //         a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        //     }
        // }
        div {
            id: "hero",
            // Math {
            //     style: "font-size:5rem; color:#fff;",
            //     tex: "{a} + {b} = {c}"
            // }

            // p {
            //     style: "font-size:5rem; color:#fff;",
            //     // "Addition and Subtraction: Up to 10"
            //     "Multiplication and Division: 2 to 5 Multiples of 10"
            // },

            p {
                style: "font-size:5rem; color:#fff; padding: 3rem;",
                "Find the number fact family using the buttons on the bottom"
            },

            for i in 0..state.facts.len() {
                FactComponent { 
                    fact: state.facts[i].clone(),
                }
            }

            div {
                style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 2rem;",

                for operand in state.operands {
                    OperandComponent { 
                        value: OperandValue::Filled(operand),
                    },
                }

                for op in state.ops {
                    OpComponent { 
                        value: OpValue::Filled(op),
                    },
                }
            }

            div {
                style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 1rem;",
                
                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 30rem; height: 6rem; line-height: 6rem; text-align: center;",
                    "Undo"
                },

                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 30rem; height: 6rem; line-height: 6rem; text-align: center;",
                    "Check"
                },
            }
        }
    }
}