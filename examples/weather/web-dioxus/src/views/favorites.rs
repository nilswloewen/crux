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
        for favorite in favorites.iter() {
            {
                let json_val = serde_json::to_value(favorite)?;
                let favorite_json = serde_json::to_string_pretty(&json_val)?;
                rsx! {
                    pre { "{favorite_json}" }
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
