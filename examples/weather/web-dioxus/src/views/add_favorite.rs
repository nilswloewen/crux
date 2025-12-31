use crate::Dispatch;
use dioxus::prelude::*;
use shared::{Event, FavoritesEvent, GeocodingResponse, ViewModel, Workflow, WorkflowViewModel};

#[component]
pub fn AddFavorite() -> Element {
    let dispatch = use_context::<Dispatch>();
    let view_model = use_context::<Signal<ViewModel>>();

    let WorkflowViewModel::AddFavorite { search_results } = view_model().workflow else {
        dispatch.send(Event::Navigate(Box::new(Workflow::AddFavorite)));
        return rsx! { "Loading..." };
    };

    let mut is_searching = use_signal(|| false);
    let mut results_at_search_start = use_signal(|| None::<Vec<GeocodingResponse>>);

    use_effect(move || {
        // Read view_model inside effect to track it as a reactive dependency
        let WorkflowViewModel::AddFavorite { search_results } = view_model().workflow else {
            return;
        };
        // Only turn off is_searching when results differ from when search started
        if is_searching() && search_results != results_at_search_start() {
            is_searching.set(false);
        }
    });

    let oninput = move |e: dioxus_core::Event<FormData>| {
        // Capture current results before starting new search
        let WorkflowViewModel::AddFavorite { search_results } = view_model().workflow else {
            return;
        };
        results_at_search_start.set(search_results);
        is_searching.set(true);
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
        if is_searching() {
            "Searching..."
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
