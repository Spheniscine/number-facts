use dioxus::prelude::*;

use crate::{components::{OperandComponent, OperandValue}, game::GameState};

#[component]
pub fn OperandEntity(value: i32, game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            onclick: move |_| { game_state.write().click_operand(value); },
            OperandComponent { 
                value: OperandValue::Filled(value),
            },
        }
        
    }
}