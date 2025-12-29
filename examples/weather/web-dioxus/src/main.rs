mod core;
mod http;

use core::CoreService;
use dioxus::prelude::*;
use serde_json;
use shared::{CurrentResponse, Event, Location, ViewModel, WeatherEvent, WorkflowViewModel};
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let view = use_signal(|| ViewModel {
        workflow: WorkflowViewModel::Home {
            weather_data: Box::new(CurrentResponse::default()),
            favorites: Vec::new(),
        },
    });

    let core = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view);
        async move { svc.run(&mut rx).await }
    });

    // send initial event
    use_resource(move || async move { core.send(Event::Home(Box::new(WeatherEvent::Show))) });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../public/css/bulma.min.css") }
        main {
            section { class: "section has-text-centered",
                h1 { class: "title", "Crux Weather Example" }
                p { class: "is-size-5", "Rust Core, Rust Shell (Dioxus)" }
            }
            section { class: "section has-text-left",
                {
                    match view().workflow {
                        WorkflowViewModel::Home { weather_data, favorites } => {
                            let json_val = serde_json::to_value(weather_data)?;
                            let weather_json = serde_json::to_string_pretty(&json_val)?;

                            rsx! {
                                h1 { class: "title", "Home" }
                                h2 { class: "subtitle", "Weather Data" }
                                pre { "{weather_json}" }
                                h2 { class: "subtitle", "Favourites" }
                                for favorite in favorites.iter() {
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
                        WorkflowViewModel::Favorites { favorites, delete_confirmation } => {
                            rsx! {
                                h1 { class: "title", "Favorites" }
                                h2 { class: "subtitle", "Favourites" }
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
                        WorkflowViewModel::AddFavorite { search_results } => {
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
                    }
                }
            }
        }
    }
}
