use crate::layout::Layout;
use crate::views::{AddFavorite, Favorites, Home};
use crate::Dispatch;
use dioxus::core::Element;
use dioxus::core_macro::{component, rsx};
use dioxus::hooks::use_context;
use dioxus::prelude::*;
use shared::{Event, FavoritesState, ViewModel, Workflow, WorkflowViewModel};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,

    #[route("/favorites")]
    Favorites,

    #[route("/favorites/add")]
    AddFavorite,
}
