use dioxus::prelude::*;

use crate::{components::Button, game::GameState};

#[component]
pub fn Undo(game_state: Signal<GameState>) -> Element {
    rsx! {
        Button { 
            onclick: move |_| { game_state.write().undo(); },
            enabled: game_state.read().facts[0].operand1.is_some(),
            text: "Undo",
        },
    }
}