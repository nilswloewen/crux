use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::hooks::{use_context, Coroutine};
use dioxus::prelude::*;
use shared::Event;
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
    let core = use_context::<Coroutine<Event>>();

    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::HomeRoute, "Home" }
                }
                li {
                    Link { to: Route::FavoritesRoute, "Favorites" }
                }
            }
        }
    }
}