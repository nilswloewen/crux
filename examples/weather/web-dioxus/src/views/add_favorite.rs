use crate::Dispatch;
use dioxus::prelude::*;
use shared::{Event, FavoritesEvent, GeocodingResponse, ViewModel, Workflow, WorkflowViewModel};

#[component]
pub fn AddFavorite(search_results: Option<Vec<GeocodingResponse>>) -> Element {
    let dispatch = use_context::<Dispatch>();
    let view_model = use_context::<Signal<ViewModel>>();

    let WorkflowViewModel::AddFavorite { search_results } = view_model().workflow else {
        dispatch.send(Event::Navigate(Box::new(Workflow::AddFavorite)));
        return rsx! { "Loading..." };
    };

    let oninput = move |e: dioxus_core::Event<FormData>| {
        dispatch.send(Event::Favorites(Box::new(FavoritesEvent::Search(
            e.value(),
        ))));
    };

    rsx! {
        Title { "Add Favorite | Crux Weather Example" }

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
    let dispatch = use_context::<Dispatch>();

    rsx! {
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
                            dispatch
                                .send(
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
                            td {
                                button { onclick, "+" }
                            }
                        }
                    }
                }
            }
        }
    }
}
