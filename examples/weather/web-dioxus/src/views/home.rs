use dioxus::prelude::*;
use shared::{CurrentResponse, FavoriteView, Favorites, WorkflowViewModel};

#[component]
pub fn Home(weather_data: Box<CurrentResponse>, favorites: Vec<FavoriteView>) -> Element {
    let json_val = serde_json::to_value(weather_data)?;
    let weather_json = serde_json::to_string_pretty(&json_val)?;

    rsx! {
        h1 { class: "title", "Home" }
        h2 { class: "subtitle", "Weather Data" }
        pre { "{weather_json}" }
        h2 { class: "subtitle", "Favourites" }

        ul {
            for favorite in favorites.iter() {
                li { "{favorite.name}" }

                {
                    let json_val = serde_json::to_value(favorite)?;
                    let favorite_json = serde_json::to_string_pretty(&json_val)?;
                    rsx! {
                        pre { "{favorite_json}" }
                    }
                }
            }
        }

    }
}
