# Dioxus implementation of Crux Weather Example

## Web (dx serve --web)

Strictly speaking, the Dioxus Router is not necessary as this could be a Single Page Application (SPA) with no browser history. 
The Router allows for updating the browser page title, URL, history, and the use of forward/back buttons.

## Development

Install the Dioxus CLI globally:

```bash
cargo install --force dioxus-cli@0.6.0-alpha.3
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

[Bulma CSS](https://bulma.io/documentation/start/overview/)