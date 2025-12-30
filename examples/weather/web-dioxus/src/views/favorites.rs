use dioxus::prelude::*;
use shared::{
    App, Core, CurrentResponse, Event, FavoriteView, FavoritesEvent, FavoritesState, Location,
    LocationOperation, LocationResult, ViewModel, WeatherEvent, Workflow, WorkflowViewModel,
};

#[component]
pub fn Favorites(favorites: Vec<FavoriteView>, delete_confirmation: Option<Location>) -> Element {
    let core = use_context::<Coroutine<Event>>();

    rsx! {
        h1 { class: "title", "Favorites" }
        h2 { class: "subtitle", "Favourites" }
        a { onclick: move |_| { core.send(Event::Navigate(Box::new(Workflow::AddFavorite))) },
            "Add Favorite"
        }
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
