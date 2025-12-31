use crate::http;
use crux_kv::{KeyValueError, KeyValueOperation, KeyValueResponse, KeyValueResult, Value};
use dioxus::prelude::{use_resource, ReadSignal, ReadableExt, ReadableOptionExt};
use dioxus::{
    prelude::{Signal, UnboundedReceiver},
    signals::WritableExt as _,
};
use dioxus_sdk_geolocation::{
    init_geolocator, use_geolocation, Error, Geocoordinates, Geolocator, PowerMode,
};
use dioxus_sdk_storage::{use_synced_storage, LocalStorage};
use futures_util::{StreamExt, TryStreamExt};
use shared::{
    App, Effect, Event, Favorites, Location, LocationOperation, LocationResult, ViewModel,
    FAVORITES_KEY,
};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use tracing::debug;
use wasm_bindgen_futures::spawn_local;

/// This simple example only needs one key "favorites", however, the crux KV API supports multiple keys, so we should too by using a HashMap.
pub type LocalStorageKV = HashMap<String, Vec<u8>>;
pub const LOCAL_STORAGE_KV_KEY: &str = "local_storage_kv";
type Core = Rc<shared::Core<App>>;

pub struct CoreService {
    core: Core,
    view: Signal<ViewModel>,
    geo: Signal<Result<Geolocator, Error>>,
    storage: Signal<LocalStorageKV>,
}

impl CoreService {
    pub fn new(
        view: Signal<ViewModel>,
        geo: Signal<Result<Geolocator, Error>>,
        storage: Signal<LocalStorageKV>,
    ) -> Self {
        Self {
            core: Rc::new(shared::Core::new()),
            view,
            geo,
            storage,
        }
    }

    pub async fn run(&self, rx: &mut UnboundedReceiver<Event>) {
        let mut view = self.view;
        view.set(self.core.view());
        while let Some(event) = rx.next().await {
            self.update(event, &mut view);
        }
    }

    fn update(&self, event: Event, view: &mut Signal<ViewModel>) {
        debug!("event: {event:?}");
        let mut st = self.storage;
        for effect in self.core.process_event(event) {
            process_effect(&self.core, effect, view, &self.geo, &mut st);
        }
    }
}

fn process_effect(
    core: &Core,
    effect: Effect,
    view: &mut Signal<ViewModel>,
    geo: &Signal<Result<Geolocator, Error>>,
    storage: &mut Signal<LocalStorageKV>,
) {
    debug!("process_effect: {:?}", effect);

    match effect {
        Effect::Render(_) => {
            view.set(core.view());
        }

        Effect::Http(mut request) => {
            spawn_local({
                let geo_clone = geo.clone();
                let mut view = view.to_owned();
                let core = core.clone();
                let mut storage_clone = storage.clone();

                async move {
                    let response = http::request(&request.operation).await;

                    for effect in core
                        .resolve(&mut request, response.into())
                        .expect("should resolve")
                    {
                        process_effect(&core, effect, &mut view, &geo_clone, &mut storage_clone);
                    }
                }
            });
        }

        Effect::KeyValue(mut request) => match &request.operation.clone() {
            KeyValueOperation::Get { ref key } => {
                let binding = storage.read();
                let res = match binding.get(key) {
                    Some(val) => KeyValueResult::Ok {
                        response: KeyValueResponse::Get {
                            value: Value::Bytes(val.clone()),
                        },
                    },
                    None => KeyValueResult::Err {
                        error: KeyValueError::CursorNotFound,
                    },
                };
                drop(binding);

                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(core, effect, view, geo, storage);
                }
            }

            KeyValueOperation::Set { ref key, ref value } => {
                let mut kv = storage.write();
                kv.insert(key.to_string(), value.clone());
                drop(kv);

                let res = KeyValueResult::Ok {
                    response: KeyValueResponse::Set {
                        previous: Value::Bytes(value.clone()),
                    },
                };
                for effect in core.resolve(&mut request, res).unwrap() {
                    process_effect(core, effect, view, geo, storage);
                }
            }
            KeyValueOperation::Delete { ref key } => {
                unimplemented!("delete")
            }
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
                    process_effect(&core, effect, view, geo, storage);
                }
            }

            LocationOperation::GetLocation => {
                let geo_clone = geo.clone();
                let mut view = view.to_owned();
                let core = core.clone();
                let mut storage_clone = storage.clone();

                spawn_local({
                    async move {
                        let binding = geo_clone.read();
                        let geolocator = match binding.deref() {
                            Ok(geo) => geo.clone(),
                            Err(e) => {
                                debug!("{e}");
                                for effect in core
                                    .resolve(&mut request, LocationResult::Location(None))
                                    .unwrap()
                                {
                                    process_effect(
                                        &core,
                                        effect,
                                        &mut view,
                                        &geo_clone,
                                        &mut storage_clone,
                                    );
                                }
                                return ();
                            }
                        };

                        let res = match geolocator.get_coordinates().await {
                            Ok(coords) => LocationResult::Location(Some(Location {
                                lat: coords.latitude,
                                lon: coords.longitude,
                            })),
                            Err(_) => LocationResult::Location(None),
                        };
                        debug!("{res:?}");

                        for effect in core.resolve(&mut request, res).unwrap() {
                            process_effect(
                                &core,
                                effect,
                                &mut view,
                                &geo_clone,
                                &mut storage_clone,
                            );
                        }
                    }
                });
            }
        },
    }
}
