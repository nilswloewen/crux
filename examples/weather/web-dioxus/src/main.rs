mod core;
mod http;
mod layout;
mod routes;
mod views;

use crate::core::{LocalStorageKV, LOCAL_STORAGE_KV_KEY};
use core::CoreService;
use crux_kv::{value, KeyValueOperation, KeyValueResponse, KeyValueResult, Value};
use dioxus::prelude::*;
use dioxus_sdk_geolocation::{init_geolocator, use_geolocation, PowerMode};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use routes::Route;
use serde_json;
use shared::{CurrentResponse, Event, ViewModel, WeatherEvent, WorkflowViewModel};
use tracing::Level;
use wasm_bindgen_futures::spawn_local;

/// Dispatch allows Dioxus to send event messages to Crux core.
pub type Dispatch = Coroutine<Event>;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let geo = init_geolocator(PowerMode::High);

    let view_model = use_signal(|| ViewModel::default());
    use_context_provider(|| view_model);

    let local_storage = use_synced_storage::<LocalStorage, LocalStorageKV>(
        LOCAL_STORAGE_KV_KEY.to_string(),
        || LocalStorageKV::new(),
    );

    let dispatch: Dispatch = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view_model.clone(), geo, local_storage);
        async move { svc.run(&mut rx).await }
    });
    use_context_provider(|| dispatch);

    // send initial event
    use_effect(move || dispatch.send(Event::Home(Box::new(WeatherEvent::Show))));

    rsx! {
        Router::<Route> {}
    }
}
