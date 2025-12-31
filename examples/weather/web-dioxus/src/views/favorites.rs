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

    if let Some(Location { lat, lon }) = delete_confirmation {
        let on_confirm =
            move |_| dispatch.send(Event::Favorites(Box::new(FavoritesEvent::DeleteConfirmed)));
        let on_cancel =
            move |_| dispatch.send(Event::Favorites(Box::new(FavoritesEvent::DeleteCancelled)));
        return rsx! {
            h2 { class: "subtitle", "Delete Confirmation" }

            "Are you sure you want to delete:"
            "Lat: {lat}, Long: {lon}"

            button { onclick: on_cancel, "cancel" }
            button { onclick: on_confirm, "Delete" }
        };
    }
    rsx! {
        Title { "Favorites | Crux Weather Example" }

        h1 { class: "title", "Favorites" }
        h2 { class: "subtitle", "Favourites" }
        Link { to: Route::AddFavorite, "Add Favorite" }
        table { class: "table",
            for favorite in favorites.iter() {
                {
                    let location = favorite.location.clone();

                    let onclick = move |_| {
                        dispatch
                            .send(
                                Event::Favorites(Box::new(FavoritesEvent::DeletePressed(location))),
                            )
                    };
                    rsx! {
                        tr {
                            th { "{favorite.name}" }
                            td { class: "has-text-right",
                                if let Some(ref current) = *favorite.current {
                                    "{current.main.temp} °C"
                                } else {
                                    "Loading..."
                                }
                            }
                            td {
                                button { onclick, "Delete" }
                            }
                        }
                    }
                }
            }
        }
    }
}
