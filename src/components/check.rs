use dioxus::prelude::*;

use crate::{components::Button, game::GameState};

#[component]
pub fn Check(game_state: Signal<GameState>) -> Element {
    rsx! {
        Button { 
            onclick: move |_| { game_state.write().check(); },
            enabled: game_state.read().is_complete(),
            text: "Check",
        },
    }
}