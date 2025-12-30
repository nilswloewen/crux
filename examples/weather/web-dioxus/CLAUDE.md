# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **Dioxus web shell** for the Crux Weather App - a Rust-based web frontend using Dioxus 0.7 that renders the shared Crux core logic in the browser via WebAssembly.

For overall project architecture, see the parent [CLAUDE.md](../CLAUDE.md).

## Build Commands

```bash
# Run dev server (hot reload)
dx serve --hot-reload

# Build for production
dx build --release

# Install Dioxus CLI (if needed)
cargo install dioxus-cli
```

## Environment Setup

The OpenWeatherMap API key must be set at build time:
```bash
export OPENWEATHER_API_KEY=your_api_key_here
dx serve
```

## Architecture

### Shell Pattern
This is a Crux "shell" - a thin platform-specific layer that:
1. Renders the `ViewModel` from the Crux core
2. Dispatches `Event`s to the core when user interacts
3. Executes `Effect`s (HTTP, storage, geolocation) that the core requests

```
User Action → Dispatch Event → Core processes → Effects returned → Shell executes → Core resolves → ViewModel updated → UI re-renders
```

### Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | App entry, context providers, initializes `CoreService` |
| `src/core.rs` | `CoreService` - runs the effect processing loop, handles all Effect types |
| `src/routes.rs` | Dioxus Router configuration (`/`, `/favorites`, `/favorites/add`) |
| `src/layout.rs` | Shared layout with navigation |
| `src/http.rs` | HTTP client using `gloo-net` |
| `src/views/` | View components (Home, Favorites, AddFavorite) |

### Key Patterns

**Dispatch Coroutine**: Events are sent to the core via a Dioxus coroutine:
```rust
pub type Dispatch = Coroutine<Event>;
let dispatch = use_context::<Dispatch>();
dispatch.send(Event::Home(Box::new(WeatherEvent::Show)));
```

**ViewModel Signal**: Shared state via Dioxus signals:
```rust
let view_model = use_context::<Signal<ViewModel>>();
```

**Effect Processing** (`core.rs`): Matches on Effect enum and handles each:
- `Effect::Render` → updates ViewModel signal
- `Effect::Http` → async HTTP request via `gloo-net`, resolves back to core
- `Effect::KeyValue` → reads/writes `LocalStorage` via `dioxus-sdk-storage`
- `Effect::Location` → uses `dioxus-sdk-geolocation` for browser geolocation

**View Guard Pattern**: Views check if the ViewModel has the right workflow, otherwise dispatch navigation:
```rust
let WorkflowViewModel::Home { weather_data, favorites } = view_model().workflow else {
    dispatch.send(Event::Navigate(Box::new(Workflow::Home)));
    return rsx! { "Loading..." };
};
```

## Key Dependencies

- `dioxus` 0.7.2 - Rust web framework with Router
- `dioxus-sdk-geolocation` - Browser geolocation API
- `dioxus-sdk-storage` - LocalStorage persistence
- `gloo-net` - HTTP requests in WASM
- `shared` - The Crux core library (local path)

## CSS

Uses [Bulma CSS](https://bulma.io/) loaded from `public/css/bulma.min.css`.
