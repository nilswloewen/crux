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
use shared::{CurrentResponse, Event, ViewModel, WeatherEvent, WorkflowViewModel};
use tracing::Level;
use wasm_bindgen_futures::spawn_local;

pub type Dispatch = Coroutine<Event>;

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let geo = init_geolocator(PowerMode::High);

    let view_model = use_signal(|| ViewModel {
        workflow: WorkflowViewModel::Home {
            weather_data: Box::new(CurrentResponse::default()),
            favorites: Vec::new(),
        },
    });
    use_context_provider(|| view_model);

    let dispatch: Dispatch = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view_model.clone(), geo);
        async move { svc.run(&mut rx).await }
    });
    use_context_provider(|| dispatch);

    // send initial event
    use_resource(move || async move { dispatch.send(Event::Home(Box::new(WeatherEvent::Show))) });

    rsx! {
        Router::<Route> {}
    }
}
