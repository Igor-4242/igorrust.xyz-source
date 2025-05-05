use dioxus::prelude::*;

mod components;
pub use components::*;

mod engine;

mod config;
pub use config::*;

mod routes;
pub use routes::*;

mod math;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    // use_context_provider(|| Signal::new(engine::Engine::new()));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/public/compound_styles.css") }
        Router::<routes::Route> {}
    }
}
