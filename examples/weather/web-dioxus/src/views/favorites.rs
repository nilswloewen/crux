use crate::{Dispatch, Route};
use dioxus::prelude::*;
use shared::{
    App, Core, CurrentResponse, Event, FavoriteView, FavoritesEvent, FavoritesState, Location,
    LocationOperation, LocationResult, ViewModel, WeatherEvent, Workflow, WorkflowViewModel,
};

#[component]
pub fn Favorites() -> Element {
    let dispatch = use_context::<Dispatch>();
    let view_model = use_context::<Signal<ViewModel>>();

    let WorkflowViewModel::Favorites {
        favorites,
        delete_confirmation,
    } = view_model().workflow
    else {
        dispatch.send(Event::Navigate(Box::new(Workflow::Favorites(
            FavoritesState::Idle,
        ))));
        return rsx! { "Loading..." };
    };

    rsx! {
        Title { "Favorites | Crux Weather Example" }

        h1 { class: "title", "Favorites" }
        h2 { class: "subtitle", "Favourites" }
        Link { to: Route::AddFavorite, "Add Favorite" }
        table { class: "table",
            for favorite in favorites.iter() {
                tr {
                    th { "{favorite.name}" }
                    td { class: "has-text-right",
                        if let Some(ref current) = *favorite.current {
                            "{current.main.temp} °C"
                        } else {
                            "Loading..."
                        }
                    }
                }
            }
        }

        h2 { class: "subtitle", "Delete Confirmation" }
        if let Some(Location { lat, lon }) = delete_confirmation {
            "Lat: {lat}, Long: {lon}"
        } else {
            "None"
        }
    }
}
