use dioxus::prelude::*;

use crate::{components::Button, game::GameState};

#[component]
pub fn Check(game_state: Signal<GameState>) -> Element {
    rsx! {
        Button { 
            onclick: move |_| { /* todo */ },
            enabled: game_state.read().is_complete(),
            text: "Check",
        },
    }
}