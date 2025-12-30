use dioxus::prelude::*;
use shared::{CurrentResponse, FavoriteView, Favorites, WorkflowViewModel};

#[component]
pub fn Home(weather_data: Box<CurrentResponse>, favorites: Vec<FavoriteView>) -> Element {
    rsx! {
        h1 { class: "title", "Home" }
        h2 { class: "subtitle", "Weather Data" }
        table { class: "table",
            tr {
                th { "{weather_data.name}" }
                td { class: "has-text-right", "{weather_data.main.temp} °C" }
            }
        }

        h2 { class: "subtitle", "Favourites" }

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
    }
}
