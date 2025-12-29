use crate::http;
use crux_kv::{KeyValueOperation, KeyValueResponse, KeyValueResult, Value};
use dioxus::prelude::{use_resource, ReadableExt};
use dioxus::{
    prelude::{Signal, UnboundedReceiver},
    signals::WritableExt as _,
};
use dioxus_sdk_geolocation::{init_geolocator, use_geolocation, PowerMode};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use futures_util::{StreamExt, TryStreamExt};
use shared::{
    App, Effect, Event, Favorites, Location, LocationOperation, LocationResult, ViewModel,
    FAVORITES_KEY,
};
use std::rc::Rc;
use tracing::debug;
use wasm_bindgen_futures::spawn_local;

type Core = Rc<shared::Core<App>>;

pub struct CoreService {
    core: Core,
    view: Signal<ViewModel>,
}

impl CoreService {
    pub fn new(view: Signal<ViewModel>) -> Self {
        Self {
            core: Rc::new(shared::Core::new()),
            view,
        }
    }

    pub async fn run(&self, rx: &mut UnboundedReceiver<Event>) {
        let mut view = self.view;
        view.set(self.core.view());
        while let Some(event) = rx.next().await {
            self.update(event, &mut view);
        }
    }

    #[tracing::instrument(skip(self, event))]
    fn update(&self, event: Event, view: &mut Signal<ViewModel>) {
        for effect in self.core.process_event(event) {
            process_effect(&self.core, effect, view);
        }
    }
}

fn process_effect(core: &Core, effect: Effect, view: &mut Signal<ViewModel>) {
    debug!("process_effect: {:?}", effect);

    match effect {
        Effect::Render(_) => {
            // This currently issues a warning:
            //
            // "Write on signal happened while a component was running.
            // Writing to signals during a render can cause infinite rerenders when you read
            // the same signal in the component. Consider writing to the signal in an
            // effect, future, or event handler if possible."
            //
            // I think this is a bug in Dioxus, as we are in a coroutine, which is a future.
            // Anyway, it works.
            view.set(core.view());
        }

        Effect::Http(mut request) => {
            spawn_local({
                let mut view = view.to_owned();
                let core = core.clone();

                async move {
                    let response = http::request(&request.operation).await;

                    for effect in core
                        .resolve(&mut request, response.into())
                        .expect("should resolve")
                    {
                        process_effect(&core, effect, &mut view);
                    }
                }
            });
        }

        Effect::KeyValue(mut request) => match request.operation {
            KeyValueOperation::Get { ref key } => {
                let local_value = use_synced_storage::<LocalStorage, Vec<u8>>(key.clone(), || Vec::new());

                let res = KeyValueResult::Ok {
                    response: KeyValueResponse::Get {
                        value: Value::Bytes(local_value()),
                    },
                };

                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(&core, effect, view);
                }
            }

            KeyValueOperation::Set { ref key, ref value } => {
                let mut local_value =
                    use_synced_storage::<LocalStorage, Vec<u8>>(key.clone(), || Vec::new());
                local_value.set(value.clone());

                let res = KeyValueResult::Ok {
                    response: KeyValueResponse::Set {
                        previous: Value::Bytes(value.clone()),
                    },
                };
                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(&core, effect, view);
                }
            }
            KeyValueOperation::Delete { key: _ } => unimplemented!("delete"),
            KeyValueOperation::Exists { key: _ } => unimplemented!("exists"),
            KeyValueOperation::ListKeys {
                prefix: _,
                cursor: _,
            } => unimplemented!("list_keys"),
        },

        Effect::Location(mut request) => match request.operation {
            LocationOperation::IsLocationEnabled => {
                let res = LocationResult::Enabled(true);

                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(&core, effect, view);
                }
            }

            LocationOperation::GetLocation => {
                let _geolocator = init_geolocator(PowerMode::High);
                let latest_coords = use_geolocation();
                let res = match latest_coords() {
                    Ok(coords) => LocationResult::Location(Some(Location {
                        lat: coords.latitude,
                        lon: coords.longitude,
                    })),
                    Err(e) => LocationResult::Location(None),
                };
                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(&core, effect, view);
                }
            }
        },
    }
}
