use dioxus::prelude::*;

use crate::game::GameState;

#[component]
pub fn Undo(game_state: Signal<GameState>) -> Element {
    rsx! {
        div {
            onclick: move |_| { game_state.write().undo(); },
            style: "border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
            width: 30rem; height: 6rem; line-height: 6rem; text-align: center;",
            "Undo",
        },
    }
}