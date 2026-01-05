use crux_core::{
    Command,
    macros::effect,
    render::{RenderOperation, render},
};
use crux_http::protocol::HttpRequest;
use crux_kv::KeyValueOperation;
use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::{
    favorites::{
        self,
        events::FavoritesEvent,
        model::{Favorite, Favorites, FavoritesState},
    },
    location::{
        Location, capability::LocationOperation, model::geocoding_response::GeocodingResponse,
    },
    navigation::{CurrentPage, NavigationTarget},
    weather::{self, events::WeatherEvent, model::current_response::CurrentResponse},
};

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum Event {
    Navigate(Box<NavigationTarget>),
    Home(Box<WeatherEvent>),
    Favorites(Box<FavoritesEvent>),
}

#[derive(Debug)]
#[effect(facet_typegen)]
pub enum Effect {
    Render(RenderOperation),
    KeyValue(KeyValueOperation),
    Http(HttpRequest),
    Location(LocationOperation),
}

#[derive(Default, Debug)]
pub struct Model {
    pub weather_data: CurrentResponse,
    pub page: CurrentPage,
    pub favorites: Favorites,
    pub search_results: Option<Vec<GeocodingResponse>>,
    pub location_enabled: bool,
    pub last_location: Option<Location>,
}

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ViewModel {
    pub workflow: WorkflowViewModel,
}

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum WorkflowViewModel {
    Home {
        weather_data: Box<CurrentResponse>,
        favorites: Vec<FavoriteView>,
    },
    Favorites {
        favorites: Vec<FavoriteView>,
        delete_confirmation: Option<Location>,
    },
    AddFavorite {
        search_results: Option<Vec<GeocodingResponse>>,
    },
}
impl Default for WorkflowViewModel {
    fn default() -> Self {
        WorkflowViewModel::Home {
            weather_data: Box::new(CurrentResponse::default()),
            favorites: Vec::new(),
        }
    }
}

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FavoriteView {
    pub name: String,
    pub location: Location,
    pub current: Box<Option<CurrentResponse>>,
}

impl From<&Favorite> for FavoriteView {
    fn from(value: &Favorite) -> Self {
        FavoriteView {
            name: value.geo.name.clone(),
            location: (&value.geo).into(),
            current: Box::new(value.current.clone()),
        }
    }
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Effect = Effect;

    fn update(&self, event: Self::Event, model: &mut Self::Model) -> Command<Effect, Event> {
        match event {
            Event::Navigate(target) => {
                // Use the type-safe navigation system
                let current = std::mem::take(&mut model.page);
                match current.try_navigate(*target) {
                    Ok(next_page) => {
                        model.page = next_page;
                        render()
                    }
                    Err(err) => {
                        // Invalid navigation - restore current page and log
                        model.page = current;
                        tracing::warn!("Invalid navigation from {} to {}", err.from, err.to);
                        Command::none()
                    }
                }
            }
            Event::Home(home_event) => {
                let mut commands = Vec::new();
                if let WeatherEvent::Show = *home_event {
                    commands.push(
                        favorites::events::update(FavoritesEvent::Restore, model)
                            .map_event(|fe| Event::Favorites(Box::new(fe))),
                    );
                }

                commands.push(
                    weather::events::update(*home_event, model)
                        .map_event(|we| Event::Home(Box::new(we))),
                );

                Command::all(commands)
            }

            Event::Favorites(fav_event) => favorites::events::update(*fav_event, model)
                .map_event(|e| Event::Favorites(Box::new(e))),
        }
    }

    fn view(&self, model: &Model) -> ViewModel {
        let favorites = model.favorites.iter().map(From::from).collect();

        let workflow = match &model.page {
            CurrentPage::Home(_) => WorkflowViewModel::Home {
                weather_data: Box::new(model.weather_data.clone()),
                favorites,
            },
            CurrentPage::Favorites(_, favorites_state) => match favorites_state {
                FavoritesState::Idle => WorkflowViewModel::Favorites {
                    favorites,
                    delete_confirmation: None,
                },
                FavoritesState::ConfirmDelete(location) => WorkflowViewModel::Favorites {
                    favorites,
                    delete_confirmation: Some(*location),
                },
            },
            CurrentPage::AddFavorite(_) => WorkflowViewModel::AddFavorite {
                search_results: model.search_results.clone(),
            },
        };

        ViewModel { workflow }
    }
}

#[cfg(test)]
mod tests {
    use crux_core::App as _;

    use super::*;

    #[test]
    fn test_navigation() {
        let app = App;
        let mut model = Model::default();

        // Navigate to Favorites
        let _ = app.update(
            Event::Navigate(Box::new(NavigationTarget::Favorites(FavoritesState::Idle))),
            &mut model,
        );

        assert!(matches!(
            model.page,
            CurrentPage::Favorites(_, FavoritesState::Idle)
        ));

        // Navigate to Home
        let _ = app.update(
            Event::Navigate(Box::new(NavigationTarget::Home)),
            &mut model,
        );
        assert!(matches!(model.page, CurrentPage::Home(_)));

        // back to favorites, so we can go to AddFavorite
        let _ = app.update(
            Event::Navigate(Box::new(NavigationTarget::Favorites(FavoritesState::Idle))),
            &mut model,
        );

        // Navigate to AddFavorite
        let _ = app.update(
            Event::Navigate(Box::new(NavigationTarget::AddFavorite)),
            &mut model,
        );

        assert!(matches!(model.page, CurrentPage::AddFavorite(_)));
    }

    #[test]
    fn test_invalid_navigation_rejected() {
        let app = App;
        let mut model = Model::default();

        // Try to navigate directly from Home to AddFavorite (invalid)
        let _ = app.update(
            Event::Navigate(Box::new(NavigationTarget::AddFavorite)),
            &mut model,
        );

        // Should still be on Home - invalid navigation was rejected
        assert!(matches!(model.page, CurrentPage::Home(_)));
    }
}
