use dioxus::prelude::*;

use crate::components::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    MainCard,
    #[route("/favorites")]
    Favorites,
}
