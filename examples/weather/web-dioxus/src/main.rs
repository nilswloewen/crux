mod core;
mod http;
mod layout;
mod routes;
mod views;

use crate::core::{LocalStorageKV, LOCAL_STORAGE_KV_KEY};
use core::CoreService;
use dioxus::prelude::*;
use dioxus_sdk_geolocation::{init_geolocator, PowerMode};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use routes::Route;
use shared::{Event, ViewModel, WeatherEvent};
use tracing::Level;

/// Dispatch allows Dioxus to send event messages to Crux core.
pub type Dispatch = Coroutine<Event>;

fn main() {
    dioxus_sdk_storage::set_dir!();
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");

    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    launch(App);
}

#[component]
fn App() -> Element {
    let geo = init_geolocator(PowerMode::High);

    let view_model = use_signal(ViewModel::default);
    use_context_provider(|| view_model);

    let local_storage = use_synced_storage::<LocalStorage, LocalStorageKV>(
        LOCAL_STORAGE_KV_KEY.to_string(),
        LocalStorageKV::new,
    );

    let dispatch: Dispatch = use_coroutine(move |mut rx| {
        let svc = CoreService::new(view_model, geo, local_storage);
        async move { svc.run(&mut rx).await }
    });
    use_context_provider(|| dispatch);

    // send initial event
    use_effect(move || dispatch.send(Event::Home(Box::new(WeatherEvent::Show))));

    rsx! {
        Router::<Route> {}
    }
}
