# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the **Weather App example** for the [Crux framework](https://github.com/redbadger/crux) - a cross-platform weather application demonstrating shared Rust business logic with iOS (SwiftUI) and Web (Dioxus) frontends.

## Build Commands

```bash
# Build the shared library
cd shared && cargo build

# Run tests (from shared directory)
cd shared && cargo test

# Run web app (requires dx CLI: cargo install dioxus-cli)
cd web-dioxus && dx serve

# Generate FFI bindings for iOS
cd shared && cargo run --features codegen --bin codegen
```

From the root Crux repository:
```bash
just build    # cargo build
just test     # cargo insta test --review --test-runner nextest --all-features --lib
just fix      # format with cargo xtask
just ci       # run CI checks
```

## Environment Setup

Set your OpenWeatherMap API key before building:
```bash
export OPENWEATHER_API_KEY=your_api_key_here
```

## Architecture

### Crux Pattern
The app follows the Crux event-driven architecture (similar to Elm):
1. **Event** → user interactions sent to core
2. **update()** → pure function processes events, updates Model, returns Commands
3. **Effect** → side-effects (HTTP, storage, location) executed by shell
4. **ViewModel** → rendered UI state

The core is **side-effect-free** and runs in WebAssembly sandbox. All side effects are managed through the Effect enum and executed by platform shells.

### Code Organization

```
shared/                      # Rust core library
├── src/
│   ├── app.rs              # App struct, Event enum, Model, ViewModel, update()
│   ├── ffi.rs              # FFI bridge (wasm-bindgen/uniffi)
│   ├── config.rs           # API key configuration
│   ├── weather/            # Weather domain
│   │   ├── events.rs       # WeatherEvent handlers + tests
│   │   └── client.rs       # WeatherApi HTTP client
│   ├── favorites/          # Favorites domain
│   │   ├── events.rs       # FavoritesEvent handlers + tests
│   │   └── model.rs        # Favorite, FavoritesState types
│   └── location/           # Location domain
│       ├── capability.rs   # Custom location capability
│       └── client.rs       # LocationApi for geocoding
web-dioxus/                  # Web UI shell (Dioxus 0.7)
├── src/
│   ├── main.rs             # Dioxus app entry point
│   ├── core.rs             # CoreService - processes effects
│   └── http.rs             # Web HTTP client (gloo-net)
iOS/                         # iOS shell (SwiftUI)
```

### Key Types (shared/src/app.rs)

```rust
pub enum Event {
    Navigate(Box<Workflow>),
    Home(Box<WeatherEvent>),
    Favorites(Box<FavoritesEvent>),
}

pub enum Effect {
    Render(RenderOperation),
    KeyValue(KeyValueOperation),
    Http(HttpRequest),
    Location(LocationOperation),
}

pub enum Workflow {
    Home,
    Favorites(FavoritesState),
    AddFavorite,
}
```

## Testing

Tests are located alongside the code in `events.rs` files. Uses **Crux's effect simulation pattern** - no mocks needed:

```rust
// Typical test pattern
let mut cmd = app.update(event, &mut model, &());
let mut request = cmd.effects().next().unwrap().expect_http();
request.resolve(HttpResult::Ok(response)).unwrap();
let next_event = cmd.events().next().unwrap();
```

Snapshot testing with `insta`:
```rust
insta::assert_yaml_snapshot!(model.weather_data);
```

## Crate Features

- `uniffi` - Mobile FFI bindings (iOS/Android)
- `wasm_bindgen` - Web WASM FFI
- `codegen` - Type generation CLI for Swift/Kotlin/TypeScript
- `facet_typegen` - Facet-based type generation

## Key Dependencies

- `crux_core` - Core framework
- `crux_http` - HTTP capability
- `crux_kv` - Key-value storage capability
- `facet` - Type generation with `#[Facet]` macro
- `dioxus` 0.7 - Web UI framework
- `derive_builder` - Builder pattern for complex types

## Workspace Info

- **Rust edition**: 2024
- **MSRV**: 1.88
- **Resolver**: 3
- Core crux crates are local paths: `../../crux_core`, `../../crux_http`, `../../crux_kv`
