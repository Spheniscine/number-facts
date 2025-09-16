use dioxus::prelude::*;

use crate::components::Math;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperandValue {
    Filled(i32), Active, Inactive
}

#[component]
pub fn OperandComponent(value: OperandValue) -> Element {
    let bgcol = if value == OperandValue::Active {"background-color: #fff;"} else {""};
    rsx! {
        if let OperandValue::Filled(value) = value {
            div {
                style: "border: 0.5rem solid #8D1CBA; background-color: #A83DD6; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
                Math { tex: "{value}" },
            },
        } else {
            div {
                style: "border: 0.5rem solid #0063B1; {bgcol} border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
                width: 10rem; height: 10rem; line-height: 10rem; text-align: center;",
            },
        }
    }
}