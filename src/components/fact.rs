use dioxus::prelude::*;

use crate::{components::{Math, OpComponent, OpValue, OperandComponent, OperandValue}, game::Fact};

#[component]
pub fn FactComponent(fact: Fact) -> Element {
    let bgcol = if fact.is_active {"background-color: #048;"} else {""};

    let mut next_active = fact.is_active;

    let operand1 = if let Some(x) = fact.operand1 {
        OperandValue::Filled(x)
    } else if next_active {
        next_active = false;
        OperandValue::Active
    } else {
        OperandValue::Inactive
    };

    let op = if let Some(x) = fact.op {
        OpValue::Filled(x)
    } else if next_active {
        next_active = false;
        OpValue::Active
    } else {
        OpValue::Inactive
    };

    let operand2 = if let Some(x) = fact.operand2 {
        OperandValue::Filled(x)
    } else if next_active {
        next_active = false;
        OperandValue::Active
    } else {
        OperandValue::Inactive
    };

    let result = if let Some(x) = fact.result {
        OperandValue::Filled(x)
    } else if next_active {
        next_active = false;
        OperandValue::Active
    } else {
        OperandValue::Inactive
    };


    rsx! {
        div {
            style: "border: 1rem solid #0063B1; {bgcol} border-radius: 1.5rem; font-size: 5rem; margin: 2rem; display: flex; flex-direction: row;",
            
            OperandComponent { value: operand1 },

            OpComponent { value: op },

            OperandComponent { value: operand2 },

            div {
                style: "font-size: 5rem; margin-top: 4rem; margin-bottom: 4rem; color:#fff; line-height: 10rem; text-align: center;",
                Math { tex: "=" },
            },

            OperandComponent { value: result },
        }
    }
}