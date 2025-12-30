use dioxus::prelude::*;
use shared::{
    App, Core, CurrentResponse, Event, FavoriteView, FavoritesEvent, FavoritesState,
    GeocodingResponse, Location, LocationOperation, LocationResult, ViewModel, WeatherEvent,
    Workflow, WorkflowViewModel,
};
#[component]
pub fn AddFavorite(search_results: Option<Vec<GeocodingResponse>>) -> Element {
    let core = use_context::<Coroutine<Event>>();

    let handle_search_input = move |e: dioxus_core::Event<FormData>| {
        core.send(Event::Favorites(Box::new(FavoritesEvent::Search(
            e.value(),
        ))))
    };

    rsx! {
        h1 { class: "title", "Add Favorite" }

        label {
            "Search"
            input { oninput: handle_search_input }
        }

        h2 { class: "subtitle", "Search Results" }
        if let Some(results) = search_results {
      SearchResults {results}
        }else {
            "No results"
        }
    }
}

#[component]
pub fn SearchResults(results: Vec<GeocodingResponse>) -> Element {
    let core = use_context::<Coroutine<Event>>();

    rsx!{
           ul {
                for result in results.iter() {
                    {
                        let res_clone = result.clone();
                        let onclick = move |_| {
                            core.send(
                                Event::Favorites(
                                    Box::new(FavoritesEvent::Submit(Box::new(res_clone.clone()))),
                                ),
                            )
                        };
                        rsx! {
                            li {
                                "{result}"
                                button { onclick, "+" }
                            }
                        }
                    }
                }
            }
   }
}