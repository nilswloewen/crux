use crate::routes::Route;
use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../public/css/bulma.min.css") }
        div { class: "container is-fluid",
            Nav {}
            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav {
            Link { to: Route::Home, "Home" }
            Link { to: Route::Favorites, "Favorites" }
        }
    }
}
