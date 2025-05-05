#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{config, math};

#[component]
pub fn MainCard() -> Element {
    rsx! {
        UnderConstruction {}

        header {

            ThiccMainRow {
                div { class: "flex flex-row items-center", gap: "34px",
                    div { class: "flex flex-row w-full items-center justify-center",
                        NavLink { href: "", "Cumqueipsum" }
                    }
                    div { class: "flex flex-row w-full items-center justify-center",
                        NavLink { href: "", "dolore" }
                    }
                    div { class: "flex flex-row w-full items-center justify-center",
                        NavLink { href: "", "Quosratione" }
                    }
                    div { class: "flex flex-row w-full items-center justify-center",
                        NavLink { href: "", "Nobissint" }
                    }
                }
            }
        }

        section {
            div {
                class: "flex flex-col md:flex-row w-full items-center justify-center",
                padding_top: "50px",
                padding_bottom: "50px",

                div {
                    class: "flex flex-col items-center justify-center",
                    width: "50%",

                    div {
                        class: "flex flex-col items-center justify-center",
                        max_width: "404px",
                        min_width: "265px",

                        AvatarAndRings {}
                    }
                }

                div { width: "50%",
                    div {
                        class: "flex flex-col",
                        gap: "28px",
                        class: "flex flex-col w-full",
                        justify_content: "stretch",
                        max_width: "404px",
                        min_width: "265px",

                        div { class: "flex flex-col",
                            H1 { "Igor Boiko" }
                            div {
                                class: "flex flex-row",
                                justify_content: "end",
                                H1 { "Rust Dev" }
                            }
                        }

                        div { class: "flex flex-col", gap: "6px",
                            H2 { "About" }
                            div { class: "flex flex-row",
                                div { width: "30%" }
                                P {
                                    "Velitlaborum odit ratione, totam, qui neque! Laborumelit qui iure culpa, tempore, ipsum. Quisamet, lorem, eveniet, velit ad dicta? Remquia, animi, odit, quasi sit quasi."
                                }
                                div { width: "30%" }
                            }
                        }

                        div { class: "flex flex-col", gap: "6px",
                            H2 { "Get in touch" }
                            div {
                                class: "flex flex-row",
                                justify_content: "end",
                                div { class: "flex flex-row", gap: "10px",
                                    P { "Gmail" }
                                    P { "Linkedin" }
                                    P { "Github" }
                                }
                            }
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
            h2 { font_size: "20px", font_weight: "semibold", color: "black", {children} }
        }
    }
}

#[component]
pub fn P(children: Element) -> Element {
    rsx! {
        div {
            p { font_size: "13px", font_weight: "light", color: "black", {children} }
        }
    }
}

#[component]
pub fn NavLink(href: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            a {
                font_size: "18px",
                font_weight: "regular",
                color: "black",
                href,
                {children}
            }
        }
    }
}

#[component]
pub fn AvatarAndRings() -> Element {
    const STEP: f32 = 60.0;
    const OFFSET: isize = 5;

    rsx! {
        div {
            class: "flex items-center justify-center",
            height: "400px",
            width: "400px",
            position: "relative",

            for i in 0..15 {
                {
                    let i: isize = OFFSET + i;
                    rsx! {
                        div {
                            position: "absolute",
                            height: format!("{}px", STEP * i as f32),
                            width: format!("{}px", STEP * i as f32),
                            background_color: if i as f32 % 2.0 == 0.0 { "black" } else { "#D9D9D9" },
                            class: "rounded-full",
                            z_index: i * -1,
                            opacity: math::map_range(0.0, 15.0, i as f32, 1.0, 0.0),
                        }
                    }
                }
            }
        }
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
        div {
            class: "flex flex-row w-full items-center justify-center",
            padding: "10px",
            background_color: "yellow",
            img {
                class: "w-10 h-10 rounded-full",
                src: config::LOGO_RUST,
                alt: "Igor Boiko Image",
            }
            div { width: "20px" }
            div {
                h1 { "🦀 Website is under construction 🦀" }
            }
        }
    }
}
