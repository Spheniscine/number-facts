use dioxus::prelude::*;

use crate::{components::Math, game::Op};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpValue {
    Filled(Op), Active, Inactive
}

#[component]
pub fn OpComponent(value: OpValue) -> Element {
    let bgcol = if value == OpValue::Active {"background-color: #fff;"} else {""};
    rsx! {
        if let OpValue::Filled(value) = value {
            div {
                style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 50%; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                Math { tex: value.to_tex() },
            },
        } else {
            div {
                style: "border: 0.5rem solid #0063B1; {bgcol} border-radius: 50%; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                    width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            },
        }
    }
}