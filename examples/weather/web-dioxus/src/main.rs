mod core;
mod http;
mod views;

use core::CoreService;
use crux_kv::{value, KeyValueOperation, KeyValueResponse, KeyValueResult, Value};
use dioxus::prelude::*;
use dioxus_sdk_geolocation::{init_geolocator, use_geolocation, PowerMode};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use serde_json;
use shared::{
    Core, CurrentResponse, Event, FavoriteView, Location, LocationOperation, LocationResult,
    ViewModel, WeatherEvent, WorkflowViewModel,
};
use tracing::Level;
use views::{AddFavorite, Favorites, Home};
use wasm_bindgen_futures::spawn_local;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

// #[derive(Clone, Debug, PartialEq, Routable)]
// enum Route {
//     #[route("/")]
//     Home {
//         weather_data: Box<CurrentResponse>,
//         favorites: Vec<FavoriteView>,
//     },
//
//     #[route("/favorites")]
//     Favorites,
//
//     #[route("/favorites/add")]
//     AddFavorite,
// }
//
// #[component]
// fn NavBar() -> Element {
//     rsx! {
//         nav {
//             Link { to: Route::Home {}, "Home" }
//         }
//         Outlet::<Route> {}
//     }
// }

#[component]
fn App() -> Element {
    let view = use_signal(|| ViewModel {
        workflow: WorkflowViewModel::Home {
            weather_data: Box::new(CurrentResponse::default()),
            favorites: Vec::new(),
        },
    });

    let geo = init_geolocator(PowerMode::High);

    let core = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view, geo);
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
                        WorkflowViewModel::Home { weather_data, favorites } => rsx! {
                            Home { weather_data, favorites }
                        },
                        WorkflowViewModel::Favorites { favorites, delete_confirmation } => {
                            rsx! {
                                Favorites { favorites, delete_confirmation }
                            }
                        }
                        WorkflowViewModel::AddFavorite { search_results } => rsx! {
                            AddFavorite { search_results }
                        },
                    }
                }
            }
        }
    }
}
