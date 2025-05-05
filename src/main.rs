use dioxus::prelude::*;

mod components;
pub use components::*;

mod config;
pub use config::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/compound_styles.css") }

        header {}

        body {
            UnderConstruction {}
            UnderConstruction {}
            UnderConstruction {}
            UnderConstruction {}
        }

        footer {}
    }
}
