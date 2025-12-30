use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::hooks::{use_context, Coroutine};
use dioxus::prelude::*;
use shared::{Event, FavoritesState, Workflow};
use crate::layout::Layout;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    HomeRoute,

    #[route("/favorites")]
    FavoritesRoute,

    #[route("/favorites/add")]
    AddFavoriteRoute,
}

#[component]
fn HomeRoute() -> Element {
    let core = use_context::<Coroutine<Event>>();
    core.send(Event::Navigate(Box::new(Workflow::Home)));

    rsx! { "Loading..." }
}

#[component]
fn FavoritesRoute() -> Element {
    let core = use_context::<Coroutine<Event>>();
    core.send(Event::Navigate(Box::new(Workflow::Favorites(
        FavoritesState::Idle,
    ))));

    rsx! { "Loading..." }
}

#[component]
fn AddFavoriteRoute() -> Element {
    let core = use_context::<Coroutine<Event>>();
    core.send(Event::Navigate(Box::new(Workflow::AddFavorite)));

    rsx! { "Loading..." }
}