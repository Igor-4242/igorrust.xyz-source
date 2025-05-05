#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::config;

#[component]
pub fn MainCard() -> Element {
    rsx! {

        header {
            ThiccMainRow {
                div { class: "flex flex-row items-center",
                    div { class: "flex flex-row w-full items-center justify-center",
                        p { "Cumqueipsum" }
                    }
                    div { width: "34px" }
                    div { class: "flex flex-row w-full items-center justify-center",
                        p { "dolore" }
                    }
                    div { width: "34px" }
                    div { class: "flex flex-row w-full items-center justify-center",
                        p { "Quosratione" }
                    }
                    div { width: "34px" }
                    div { class: "flex flex-row w-full items-center justify-center",
                        p { "Nobissint" }
                    }
                }
            }
        }

        UnderConstruction {}

        body {
            div { class: "flex flex-col w-full items-center justify-center",
                AvatarAndRings {}
                div { class: "flex flex-col w-full", justify_content: "stretch",
                    div {
                        h1 { "Igor Boiko" }
                        div {
                            h1 { "Rust Dev" }
                        }
                    }
                    div {
                        h2 { "About" }
                        div {
                            p {
                                "Velitlaborum odit ratione, totam, qui neque! Laborumelit qui iure culpa, tempore, ipsum. Quisamet, lorem, eveniet, velit ad dicta? Remquia, animi, odit, quasi sit quasi."
                            }
                        }
                    }
                    div {
                        h2 { "Get in touch" }
                        div {
                            p { "Gmail" }
                            p { "Linkedin" }
                            p { "Github" }
                        }
                    }
                }
            }
        }

        footer {
            ThiccMainRow {
                p {
                    "Cumqueipsum dolore nihil at, rem sunt. Quosratione, ipsum ad unde quo, ut? Sitquae vitae, laborum ad, cumque et. Nobissint dolores sed, illo dicta, sunt. "
                }
            }
        }
    }
}

#[component]
pub fn ThiccMainRow(children: Element) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full items-center justify-center px-40",
            padding_top: "20px",
            padding_bottom: "20px",
            padding_left: "20%",
            padding_right: "20%",
            {children}
        }
    }
}

#[component]
pub fn Favorites() -> Element {
    rsx! { "favorites!" }
}

#[component]
pub fn H1(children: Element) -> Element {
    rsx! {
        div {
            h1 { font_size: "48px", font_weight: "extrabold", color: "black", {children} }
        }
    }
}

#[component]
pub fn H2(children: Element) -> Element {
    rsx! {
        div {
            h1 { font_size: "20px", font_weight: "semibold", color: "black", {children} }
        }
    }
}

#[component]
pub fn P(children: Element) -> Element {
    rsx! {
        div {
            h1 { font_size: "13px", font_weight: "light", color: "black", {children} }
        }
    }
}

#[component]
pub fn AvatarAndRings() -> Element {
    rsx! {
        div { height: "100px", width: "100px", background_color: "red" }
    }
}

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
