use dioxus::prelude::*;
use rand::{rng, Rng};

use crate::{components::{operand::OperandValue, FactComponent, Math, OperandComponent}, game::{Fact, GameState, Op}};

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

            for i in 0..4 {
                FactComponent { 
                    fact: state.facts[i].clone(),
                }
            }

            // FactComponent {
            //     fact: test_fact
            // },

            // for _ in 0..3 {
            //     div {
            //         style: "border: 1rem solid #0063B1; border-radius: 1.5rem; font-size: 5rem; margin: 2rem; display: flex; flex-direction: row;",
                    
            //         div {
            //             style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 1rem; font-size: 5rem; margin: 3rem; padding: 2rem; color:#fff;
            //             width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            //             Math { tex: "111" },
            //         },

            //         div {
            //             style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 50%; font-size: 5rem; margin: 3rem; padding: 2rem; color:#fff;
            //             width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            //             Math { tex: "+" },
            //         },

            //         div {
            //             style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 1rem; font-size: 5rem; margin: 3rem; padding: 2rem; color:#fff;
            //             width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            //             Math { tex: "222" },
            //         },

            //         div {
            //             style: "font-size: 5rem; margin-top: 5rem; margin-bottom: 5rem; color:#fff; line-height: 10rem; text-align: center;",
            //             Math { tex: "=" },
            //         },

            //         div {
            //             style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 1rem; font-size: 5rem; margin: 3rem; padding: 2rem; color:#fff;
            //             width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            //             Math { tex: "333" },
            //         },
            //     }
            // }

            div {
                style: "font-size: 5rem; display: flex; flex-direction: row; margin-top: 2rem;",
                
                OperandComponent { 
                    value: OperandValue::Inactive,
                },

                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                    Math { tex: "222" },
                },

                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                    Math { tex: "333" },
                },

                
                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 50%; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                    Math { tex: "+" },
                },

                
                div {
                    style: "border: 0.5rem solid #0063B1; border-radius: 50%; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                    Math { tex: "-" },
                },
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