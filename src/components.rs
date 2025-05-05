#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::config;

#[component]
pub fn StacksIcon() -> Element {
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-10 h-10 text-white p-2 bg-indigo-500 rounded-full",
            view_box: "0 0 24 24",
            path { d: "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" }
        }
    }
}

#[component]
pub fn RightArrowIcon() -> Element {
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            class: "w-4 h-4 ml-1",
            view_box: "0 0 24 24",
            path { d: "M5 12h14M12 5l7 7-7 7" }
        }
    }
}

#[component]
pub fn UnderConstruction() -> Element {
    rsx! {
        div { class: "flex flex-col w-full items-center justify-center",
            img {
                class: "w-10 h-10 rounded-full",
                src: config::LOGO_RUST,
                alt: "Igor Boiko Image",
            }
            div {
                h1 { "🦀 Under construction 🦀" }
            }
        }
    }
}
