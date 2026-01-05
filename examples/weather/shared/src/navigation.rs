//! Type-safe navigation using phantom types.
//!
//! This module enforces valid page transitions at compile time. Each `Page<T>`
//! type only has methods for valid transitions from that page:
//!
//! - `Page<Home>` → can go to `Favorites`
//! - `Page<Favorites>` → can go to `AddFavorite` or `Home`
//! - `Page<AddFavorite>` → can go to `Favorites` or `Home`
//!
//! Invalid transitions (e.g., `Home` → `AddFavorite`) simply don't have methods,
//! so they cause compile errors if attempted.

use std::marker::PhantomData;

use facet::Facet;
use serde::{Deserialize, Serialize};

use crate::{favorites::model::FavoritesState, location::Location};

// Page type markers
#[derive(Debug, Default, Clone)]
pub struct Home;
#[derive(Debug, Default, Clone)]
pub struct Favorites;
#[derive(Debug, Default, Clone)]
pub struct AddFavorite;

/// A phantom-typed page that enforces valid transitions at compile time.
/// Only has navigation methods for valid transitions from this page type.
#[derive(Debug, Clone)]
pub struct Page<P> {
    _marker: PhantomData<P>,
}

impl<P> Default for Page<P> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

// === Type-safe transitions ===
// Each impl block defines only the valid transitions FROM that page type.
// Missing methods = compile-time prevention of invalid transitions.

impl Page<Home> {
    /// Navigate from Home to Favorites (valid transition)
    pub fn to_favorites(self, state: FavoritesState) -> CurrentPage {
        CurrentPage::Favorites(Page::default(), state)
    }
    // Note: No to_add_favorite() method - that transition is invalid
}

impl Page<Favorites> {
    /// Navigate from Favorites to AddFavorite (valid transition)
    pub fn to_add_favorite(self) -> CurrentPage {
        CurrentPage::AddFavorite(Page::default())
    }

    /// Navigate from Favorites to Home (valid transition)
    pub fn to_home(self) -> CurrentPage {
        CurrentPage::Home(Page::default())
    }

    /// Update the state within Favorites (e.g., show delete confirmation)
    pub fn with_state(self, state: FavoritesState) -> CurrentPage {
        CurrentPage::Favorites(Page::default(), state)
    }
}

impl Page<AddFavorite> {
    /// Navigate from AddFavorite back to Favorites (valid transition)
    pub fn to_favorites(self, state: FavoritesState) -> CurrentPage {
        CurrentPage::Favorites(Page::default(), state)
    }

    /// Navigate from AddFavorite to Home (valid transition)
    pub fn to_home(self) -> CurrentPage {
        CurrentPage::Home(Page::default())
    }
    // Note: No to_add_favorite() - already there
}

/// The current page state, carrying both the phantom-typed page and any associated data.
#[derive(Debug, Clone)]
pub enum CurrentPage {
    Home(Page<Home>),
    Favorites(Page<Favorites>, FavoritesState),
    AddFavorite(Page<AddFavorite>),
}

impl Default for CurrentPage {
    fn default() -> Self {
        Self::Home(Page::default())
    }
}

/// Target page for navigation events from the shell.
/// This is the "request" type - actual transitions are validated against current page.
#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum NavigationTarget {
    #[default]
    Home,
    Favorites(FavoritesState),
    AddFavorite,
}

/// Error returned when an invalid navigation is attempted.
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidNavigation {
    pub from: &'static str,
    pub to: &'static str,
}

impl CurrentPage {
    /// Attempt to navigate to a target page.
    /// Returns the new page if the transition is valid, or an error if not.
    ///
    /// This method uses the phantom-typed Page methods internally,
    /// ensuring that valid transitions are enforced at compile time
    /// in the implementation, while gracefully handling invalid
    /// requests from the shell at runtime.
    pub fn try_navigate(self, target: NavigationTarget) -> Result<CurrentPage, InvalidNavigation> {
        match (self, target) {
            // From Home
            (CurrentPage::Home(_), NavigationTarget::Home) => Ok(CurrentPage::Home(Page::default())),
            (CurrentPage::Home(p), NavigationTarget::Favorites(state)) => Ok(p.to_favorites(state)),
            (CurrentPage::Home(_), NavigationTarget::AddFavorite) => Err(InvalidNavigation {
                from: "Home",
                to: "AddFavorite",
            }),

            // From Favorites
            (CurrentPage::Favorites(p, _), NavigationTarget::Home) => Ok(p.to_home()),
            (CurrentPage::Favorites(p, _), NavigationTarget::Favorites(state)) => {
                Ok(p.with_state(state))
            }
            (CurrentPage::Favorites(p, _), NavigationTarget::AddFavorite) => Ok(p.to_add_favorite()),

            // From AddFavorite
            (CurrentPage::AddFavorite(p), NavigationTarget::Home) => Ok(p.to_home()),
            (CurrentPage::AddFavorite(p), NavigationTarget::Favorites(state)) => {
                Ok(p.to_favorites(state))
            }
            (CurrentPage::AddFavorite(_), NavigationTarget::AddFavorite) => {
                Ok(CurrentPage::AddFavorite(Page::default()))
            }
        }
    }

    /// Update the favorites state without changing the page.
    /// Only valid when on the Favorites page.
    pub fn update_favorites_state(&mut self, state: FavoritesState) {
        if let CurrentPage::Favorites(_, current_state) = self {
            *current_state = state;
        }
    }

    /// Get the current favorites state, if on the Favorites page.
    pub fn favorites_state(&self) -> Option<&FavoritesState> {
        match self {
            CurrentPage::Favorites(_, state) => Some(state),
            _ => None,
        }
    }

    /// Check if currently showing delete confirmation for a specific location.
    pub fn is_confirming_delete(&self, location: &Location) -> bool {
        matches!(
            self,
            CurrentPage::Favorites(_, FavoritesState::ConfirmDelete(loc)) if loc == location
        )
    }
}
