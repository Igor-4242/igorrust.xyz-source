use dioxus::prelude::*;

mod components;
use components::*;

static AVATAR: Asset = asset!("/public/main__avatar.png");

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
    // div {
    //     class: "text-gray-400 body-font",
    //     div {
    //         class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
    //         nav {
    //             class: "md:ml-auto flex flex-wrap items-center text-base justify-center",
    //             a {
    //                 class: "mr-5 hover:text-white",
    //                 href: "mailto:igorboiko4242@gmail.com",
    //                 "Gmail"
    //             }
    //             a {
    //                 class: "mr-5 hover:text-white",
    //                 href: "https://in/igor-boiko-538524332",
    //                 "Linkedin"
    //             }
    //             a {
    //                 class: "mr-5 hover:text-white",
    //                 href: "https://github.com/Igor-4242",
    //                 "Github"
    //             }
    //         }
    //         a {
    //             href: "mailto:igorboiko4242@gmail.com",
    //             button {
    //                 class: "inline-flex items-center bg-green-800 py-1 px-3 focus:outline-none hover:bg-gray-700 rounded text-base mt-4 md:mt-0",
    //                 "Contact me"
    //                 RightArrowIcon {}
    //             }
    //         }
    //     }


        div {
            class: "flex flex-col w-full items-center justify-center",
            img {
                class: "w-10 h-10 rounded-full",
                src: AVATAR,
                alt: "Igor Boiko Image",
            }
            div {
                h1 { "🦀 Under construction 🦀"}
                // h1 { "Igor Boiko"}
                // h3 { "🦀 Developer" }
            }
        }

    }
}
