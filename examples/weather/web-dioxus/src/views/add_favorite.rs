use dioxus::prelude::*;
use shared::{
    App, Core, CurrentResponse, Event, FavoriteView, FavoritesEvent, FavoritesState,
    GeocodingResponse, Location, LocationOperation, LocationResult, ViewModel, WeatherEvent,
    Workflow, WorkflowViewModel,
};

#[component]
pub fn AddFavorite(search_results: Option<Vec<GeocodingResponse>>) -> Element {
    let core = use_context::<Coroutine<Event>>();

    let oninput = move |e: dioxus_core::Event<FormData>| {
        core.send(Event::Favorites(Box::new(FavoritesEvent::Search(
            e.value(),
        ))))
    };

    rsx! {
        h1 { class: "title", "Add Favorite" }

        div { class: "field",
            label { class: "label", "Search" }
            div { class: "control",
                input { class: "input", oninput }
            }
        }

        h2 { class: "subtitle", "Search Results" }
        if let Some(results) = search_results {
            SearchResults { results }
        } else {
            "No results"
        }
    }
}

#[component]
pub fn SearchResults(results: Vec<GeocodingResponse>) -> Element {
    let core = use_context::<Coroutine<Event>>();

    rsx!{
        table { class: "table", class: "is-striped",
            thead {
                tr {
                    th { "Name" }
                    th { "Country" }
                    th { "State" }
                    th { "Add" }
                }
            }
            for result in results.iter() {
                tr {
                    {
                        let res_clone = result.clone();
                        let onclick = move |_| {
                            core.send(
                                Event::Favorites(
                                    Box::new(FavoritesEvent::Submit(Box::new(res_clone.clone()))),
                                ),
                            )
                        };
                        let state = result.state.clone().unwrap_or_default();

                        rsx! {
                            td { "{result.name}" }
                            td { "{result.country}" }
                            td { "{state}" }
                            td { button { onclick, "+" } }
                        }
                    }
                }
            }
        }
    }
}