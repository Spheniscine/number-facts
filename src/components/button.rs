use dioxus::prelude::*;

#[component]
pub fn Button(onclick: EventHandler<MouseEvent>, enabled: bool, text: String) -> Element {
    rsx! {
        div {
            onclick,
            style: if enabled {"border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#fff;
            width: 30rem; height: 6rem; line-height: 6rem; text-align: center;"}
                else {"border: 0.5rem solid #0063B1; border-radius: 1rem; font-size: 5rem; margin: 2rem; padding: 2rem; color:#99b;
                    width: 30rem; height: 6rem; line-height: 6rem; text-align: center; background-color: #779;"},
            "{text}",
        },
    }
}