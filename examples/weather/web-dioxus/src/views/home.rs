use dioxus::prelude::*;
use shared::{
    CurrentResponse, Event, FavoriteView, Favorites, ViewModel, Workflow, WorkflowViewModel,
};
use crate::Dispatch;

#[component]
pub fn Home() -> Element {
    let dispatch = use_context::<Dispatch>();
    let view_model = use_context::<Signal<ViewModel>>();

    let WorkflowViewModel::Home {
        weather_data,
        favorites,
    } = view_model().workflow
    else {
        dispatch.send(Event::Navigate(Box::new(Workflow::Home)));
        return rsx! { "Loading..." };
    };

    rsx! {
        Title { "Home | Crux Weather Example" }

        h1 { class: "title", "Home" }
        h2 { class: "subtitle", "Current Location" }
        table { class: "table",
            tr {
                th { "{weather_data.name}" }
                td { class: "has-text-right", "{weather_data.main.temp} °C" }
            }
        }

        h2 { class: "subtitle", "Favourites" }

        table { class: "table",
            for favorite in favorites.iter() {
                tr {
                    th { "{favorite.name}" }
                    td { class: "has-text-right",
                        if let Some(ref current) = *favorite.current {
                            "{current.main.temp} °C"
                        } else {
                            "Loading..."
                        }
                    }
                }
            }
        }
    }
}
