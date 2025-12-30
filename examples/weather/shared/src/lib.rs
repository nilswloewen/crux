#![allow(clippy::missing_panics_doc)]

use crux_core::bridge::EffectId;
pub use crux_core::bridge::{Bridge, Request};
pub use crux_core::{Core, ResolveError};
pub use crux_http as http;

mod app;
mod config;
mod favorites;
#[cfg(any(feature = "wasm_bindgen", feature = "uniffi"))]
mod ffi;
mod location;
mod navigation;
mod weather;

pub use app::{App, Effect, Event, FavoriteView, Model, ViewModel, Workflow, WorkflowViewModel};
pub use favorites::events::FavoritesEvent;
pub use favorites::model::{FAVORITES_KEY, Favorite, Favorites, FavoritesState};
pub use location::capability::{LocationOperation, LocationResult};
pub use location::{GeocodingResponse, Location};
pub use weather::events::WeatherEvent;
pub use weather::model::current_response::CurrentResponse;

#[cfg(any(feature = "wasm_bindgen", feature = "uniffi"))]
pub use ffi::CoreFFI;

#[cfg(feature = "uniffi")]
const _: () = assert!(
    uniffi::check_compatible_version("0.29.4"),
    "please use uniffi v0.29.4"
);
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
