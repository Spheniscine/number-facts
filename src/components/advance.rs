use dioxus::prelude::*;

use crate::{components::Button, game::GameState};

#[component]
pub fn Advance(game_state: Signal<GameState>) -> Element {
    rsx! {
        Button { 
            onclick: move |_| { game_state.write().advance(); },
            enabled: true,
            text: if game_state.read().is_correct() {"Continue"} else {"Try Again"},
        },
    }
}