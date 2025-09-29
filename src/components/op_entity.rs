use dioxus::prelude::*;

use crate::{components::{OpComponent, OpValue}, game::{GameState, Op}};

#[component]
pub fn OpEntity(value: Op, game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            onclick: move |_| { game_state.write().click_op(value); },
            OpComponent { 
                value: OpValue::Filled(value),
            },
        }
        
    }
}