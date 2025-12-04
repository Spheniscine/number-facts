use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::{components::Math, game::{addition_difficulties, multiplication_difficulties, Feedback, GameState, Op, ScreenState, SettingsState, StringKind, NAME_DIFFICULTY_CHOICE, NAME_OP, VALUE_OP_PLUS, VALUE_OP_TIMES}};

#[component]
pub fn RadioButton(mut state: Signal<SettingsState>, name: String, value: String, checked: bool, width: Option<f64>, children: Element) -> Element {
    let name_ref = name.clone(); let value_ref = value.clone();
    let onchange = move |_| {
        state.write().parse_radio_button_change(&name_ref, &value_ref);
    };
    let style = if let Some(width) = width {format!("width: {width}rem;")} else {String::new()};

    rsx! {
        label {
            input {
                r#type: "radio",
                name: {name.to_string()},
                value: {value.to_string()},
                checked,
                onchange,
            },
            span { 
                style,
                class: "inner",
                {children}
            },
        },
    }
}

#[component]
pub fn Settings(game_state: Signal<GameState>) -> Element {

    let mut state = use_signal(|| game_state.read().get_settings_state());

    let difficulties = if state.read().difficulty_options.op == Op::Plus {addition_difficulties()} else {multiplication_difficulties()};

    let reset_level_changed = move |evt: Event<FormData>| {
        state.write().reset_level = evt.checked();
    };

    let audio_settings_changed = move |evt: Event<FormData>| {
        state.write().audio_state = evt.value().parse().unwrap_or(100);
    };

    let do_ok = move |mut game_state: Signal<GameState>| {
        let not_first = game_state.read().settings_cancelable;
        game_state.write().apply_settings(state.read().clone());
        game_state.write().screen_state = ScreenState::Game;
    };
    
    let mut ok = move |_| {
        do_ok(game_state.clone())
    };
    let mut cancel = move |_| {
        game_state.write().screen_state = ScreenState::Game;
    };

    let mut onmounted = async move |e: Event<MountedData>| {
        e.set_focus(true).await;
    };
    let mut onkeydown = move |e: Event<KeyboardData>| {
        let key = e.key();
        match key {
            Key::Enter => {
                do_ok(game_state.clone())
            }
            Key::Escape => {
                if game_state.read().settings_cancelable {
                    game_state.write().screen_state = ScreenState::Game;
                }
            }
            _ => {}
        }
    };

    rsx! {
        style {
            "#settingsDialog:focus {{ outline: none; }}"
        }
        div {
            id: "settingsDialog",
            style: "margin: 2%; padding: 5rem; width: 87%; height: 92%; background-color: #ccc; font-size: 5rem;
                line-height: 9rem;
                color: #000; border-radius: 2rem;",
            tabindex: -1,
            onmounted: onmounted,
            onkeydown: onkeydown,
            p {
                class: "radio-buttons",
                "Operations: ",
                RadioButton {  
                    state,
                    name: NAME_OP,
                    value: VALUE_OP_PLUS,
                    checked: state.read().difficulty_options.op == Op::Plus,
                    Math {tex: "+"}, " and ", Math {tex: "-"},
                },
                " "
                RadioButton { 
                    state, 
                    name: NAME_OP,
                    value: VALUE_OP_TIMES,
                    checked: state.read().difficulty_options.op == Op::Times,
                    Math {tex: "\\times"}, " and ", Math {tex: "\\div"},
                },
            },
            p {
                class: "radio-buttons",
                for i in 0..difficulties.len() {
                    if i > 0 {
                        br {}
                    }
                    RadioButton { 
                        state,
                        name: NAME_DIFFICULTY_CHOICE,
                        value: "{i}",
                        checked: state.read().difficulty_options == difficulties[i],
                        width: 80.,

                        for (kind, st) in &difficulties[i].description {
                            if *kind == StringKind::Normal {
                                "{st}"
                            } else {
                                Math {
                                    tex: st,
                                }
                            }
                        }
                    }
                }
            },

            p { 
                "Generate new problems: ",
                input {
                    r#type: "checkbox",
                    style: "width: 4rem; height: 4rem;",
                    checked: state.read().reset_level,
                    disabled: !game_state.read().settings_cancelable || state.read().difficulty_options != game_state.read().difficulty,
                    onchange: reset_level_changed
                }
            },

            p { 
                "Audio: ",
                input {
                    r#type: "range",
                    style: "width: 50rem; height: 4rem;",
                    min: 0, max: 100, step: 5,
                    value: state.read().audio_state,
                    oninput: audio_settings_changed
                },
                " {state.read().audio_state}",
            },

            p { 
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: ok,
                    "OK"
                },
                " ",
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    onclick: cancel,
                    "Cancel"
                },
            },

            p {
                style: "position: absolute; bottom: 1.5rem; font-size: 3rem;",
                "© OnlineMathLearning.com"
            },
        }
    }
}