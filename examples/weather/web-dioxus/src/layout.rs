use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::hooks::use_context;
use dioxus::prelude::*;
use crate::Dispatch;
use crate::routes::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../public/css/bulma.min.css") }
        Nav {}
        Outlet::<Route> {}
    }
}

#[component]
pub fn Nav() -> Element {
    let _dispatch = use_context::<Dispatch>();

    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home, "Home" }
                }
                li {
                    Link { to: Route::Favorites, "Favorites" }
                }
            }
        }
    }
}