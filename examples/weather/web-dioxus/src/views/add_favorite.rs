use dioxus::prelude::*;
use shared::{CurrentResponse, FavoriteView, Favorites, GeocodingResponse, WorkflowViewModel};

#[component]
pub fn AddFavorite(search_results: Option<Vec<GeocodingResponse>>) -> Element {
    rsx! {
        h1 { class: "title", "Add Favorite" }
        h2 { class: "subtitle", "Search Results" }
        if let Some(results) = search_results {
            for result in results.iter() {
                "{result}"
            }
        }
    }
}
