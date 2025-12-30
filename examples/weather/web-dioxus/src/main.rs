mod core;
mod http;
mod layout;
mod routes;
mod views;

use core::CoreService;
use crux_kv::{value, KeyValueOperation, KeyValueResponse, KeyValueResult, Value};
use dioxus::prelude::*;
use dioxus_sdk_geolocation::{init_geolocator, use_geolocation, PowerMode};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use routes::Route;
use serde_json;
use shared::{
    Core, CurrentResponse, Event, FavoriteView, FavoritesEvent, FavoritesState, GeocodingResponse,
    Location, LocationOperation, LocationResult, ViewModel, WeatherEvent, Workflow,
    WorkflowViewModel,
};
use tracing::Level;
use views::{AddFavorite, Favorites, Home};
use wasm_bindgen_futures::spawn_local;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let view_model = use_signal(|| ViewModel {
        workflow: WorkflowViewModel::Home {
            weather_data: Box::new(CurrentResponse::default()),
            favorites: Vec::new(),
        },
    });

    let geo = init_geolocator(PowerMode::High);

    let crux = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view_model.clone(), geo);
        async move { svc.run(&mut rx).await }
    });
    use_context_provider(|| crux);

    // send initial event
    use_resource(move || async move { crux.send(Event::Home(Box::new(WeatherEvent::Show))) });

    rsx! {
        Router::<Route> {}

        {
            match view_model().workflow {
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
