use std::rc::Rc;

use dioxus::{logger::tracing, prelude::*};
use strum::IntoEnumIterator;

use crate::{components::Math, game::{addition_difficulties, multiplication_difficulties, Feedback, GameState, Op, ScreenState, StringKind}};

#[component]
pub fn RadioButton(name: String, value: String, checked: bool, width: Option<f64>, children: Element) -> Element {
    // let name_ref = name.clone(); let value_ref = value.clone();
    // let onchange = move |_| {
    //     state.write().difficulty_options.insert(name_ref.to_string(), value_ref.to_string());
    //     let addend_limit = state.read().addend_limit();
    //     let max_addend = state.read().difficulty_options[LegacyDifficulty::STR_ADDEND_RANGE].rsplit(',').next().unwrap().parse::<i32>().unwrap();
    //     if max_addend > addend_limit { 
    //         state.write().difficulty_options.insert(LegacyDifficulty::STR_ADDEND_RANGE.into(), "1,1".into());
    //     }
    // };
    let style = if let Some(width) = width {format!("width: {width}rem;")} else {String::new()};

    rsx! {
        label {
            input {
                r#type: "radio",
                name: {name.to_string()},
                value: {value.to_string()},
                checked,
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
            // onmounted: onmounted,
            // onkeydown: onkeydown,
            // if let Difficulty::Legacy(difficulty) = game_state.read().difficulty {
                p {
                    class: "radio-buttons",
                    "Operations: ",
                    RadioButton {  
                        name: "op",
                        value: "plus",
                        checked: state.read().difficulty_options.op == Op::Plus,
                        Math {tex: "+"}, " and ", Math {tex: "-"},
                    },
                    " "
                    RadioButton {  
                        name: "op",
                        value: "times",
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
                            name: "difficulty_choice",
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
                    // checked: state.read().reset_level,
                    // disabled: !game_state.read().settings_cancelable || state.read().difficulty_options != game_state.read().difficulty.to_map(),
                    // onchange: reset_level_changed
                }
            },

            p { 
                "Audio: ",
                input {
                    r#type: "range",
                    style: "width: 50rem; height: 4rem;",
                    min: 0, max: 100, step: 5, 
                    value: 100,
                    // value: state.read().audio_state,
                    // oninput: audio_settings_changed
                },
                " XX",
            },

            p { 
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    // onclick: ok,
                    "OK"
                },
                " ",
                button {
                    r#type: "button",
                    style: "width: 20rem; font-size: 5rem; font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;",
                    // onclick: cancel,
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