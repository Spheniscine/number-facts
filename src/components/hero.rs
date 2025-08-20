use dioxus::prelude::*;
use rand::{rng, Rng};

use crate::components::Math;

#[component]
pub fn Hero() -> Element {
    let rng = &mut rng();
    let a = rng.random_range(0..10);
    let b = rng.random_range(0..10);
    let c = a + b;
    rsx! {
        // div {
        //     id: "hero",
        //     img { src: HEADER_SVG, id: "header" }
        //     div { id: "links",
        //         a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
        //         a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
        //         a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
        //         a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
        //         a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
        //         a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        //     }
        // }
        div {
            id: "hero",
            Math {
                style: "font-size:5rem; color:#fff;",
                tex: "{a} + {b} = {c}"
            }
        }
    }
}