<div align="center">

# sawe

**A compact toolkit for building modern web applications with [Salvo](https://salvo.rs).**

[![Crates.io](https://img.shields.io/crates/v/sawe?style=for-the-badge)](https://crates.io/crates/sawe)
[![Documentation](https://img.shields.io/docsrs/sawe?style=for-the-badge)](https://docs.rs/sawe)
[![License](https://img.shields.io/crates/l/sawe?style=for-the-badge)](https://github.com/p1mo/sawe/blob/dev/LICENSE_MIT)

HTTP/3 · TLS · Handlebars · Embedded assets · UI icons · SEO · OpenAPI

</div>

---

## What is sawe?

`sawe` brings together a set of useful building blocks around the [Salvo](https://salvo.rs) web framework. It keeps the flexibility of Salvo while providing a convenient server wrapper and optional tools for rendering templates, working with assets and icons, generating search metadata, and exposing OpenAPI documentation.

Use only the pieces you need: the core server is available by default, while the richer template and documentation APIs are enabled through Cargo features.

## Highlights

- **Modern web serving** — run Salvo applications over HTTP/3 (QUIC) and HTTP/2 or HTTP/1 over TCP.
- **TLS out of the box** — load an existing PEM certificate/key pair or generate a local self-signed certificate on first run.
- **Graceful shutdown** — handle Ctrl+C and termination signals, with an optional shutdown timeout.
- **Handlebars templates** — render templates from embedded or filesystem-backed assets.
- **Asset and icon access** — use UFS-backed files and UI-icon lookups from Salvo handlers.
- **SEO helpers** — compose standard meta tags, Open Graph and Twitter cards, plus Schema.org JSON-LD data.
- **OpenAPI integration** — attach OpenAPI routers and render the accompanying documentation when the relevant features are enabled.
- **Salvo-native** — `salvo` and `tokio` are re-exported for a convenient, consistent API surface.

## Installation

The crate uses Rust edition 2024. Use a Rust toolchain with edition 2024 support.

Add the core crate:

```bash
cargo add sawe
```

Enable template rendering, UFS assets, and UI-icon access:

```bash
cargo add sawe --features templates
```

Or configure features directly in `Cargo.toml`:

```toml
[dependencies]
sawe = { version = "0.1.1", features = ["templates"] }
```

## Quick start: start a server

`WebServer` takes a Salvo router, an error catcher, an optional graceful-shutdown timeout, a bind address, and a directory for TLS material.

```rust
use sawe::{
    salvo::{catcher::Catcher, prelude::Router},
    server::WebServer,
    tokio,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sawe::server::ServerError> {
    let router = Router::new();

    let server = WebServer::new(
        router,
        Catcher::default(),
        Some(Duration::from_secs(10)),
        ("127.0.0.1", 5800),
        "./.sawe/tls",
    );

    server.run().await
}
```

Add your handlers and routes to the `Router` before passing it to `WebServer::new`.

### TLS behaviour

On first start, `sawe` creates the TLS directory and generates a self-signed certificate if neither of the expected files exists:

```text
.sawe/tls/
├── certs.pem
└── key.pem
```

The server enforces HTTPS. For local development, open `https://127.0.0.1:5800` (or the host and port you configured). Your browser will warn that the automatically generated certificate is not trusted; that is expected.

**For production, provide a trusted certificate and matching private key** using the filenames above. The built-in generated certificate is intended for local hosts, not a publicly trusted deployment. Keep `key.pem` private and out of version control.

## Optional features

`sawe` uses Cargo features so applications can opt into the additional dependencies only when they need them.

| Feature | What it enables |
| --- | --- |
| *(no feature)* | Core `WebServer`, Salvo and Tokio re-exports, and shared error types. |
| `templates` | Handlebars rendering, UFS-backed template entries, template helpers, and UI-icon access. |
| `docs` | OpenAPI router integration and documentation-store helpers. Enable `templates` alongside this feature for the Handlebars-backed documentation renderer. |
| `build` | UI-icon build support; the `build_uiicons` re-export is available when used together with `templates`. |
| `google-type-dir` | Additional Google-oriented Schema.org type definitions for structured data. |

Enable multiple features as needed:

```toml
[dependencies]
sawe = {
    version = "0.1.1",
    features = ["templates", "docs", "build", "google-type-dir"],
}
```

## Templates and assets

Enable the `templates` feature to use the `sawe::templates` module. Its main pieces are:

- `Entry::new_static(path)` — read templates from a directory on disk.
- `Entry::new_embed(fs)` — use an embedded `ufs::UnifiedFS` asset collection.
- `Templates<T>` — register entries under application-defined keys and find or render files using glob patterns.
- `TemplatesDepotExt` — convenience methods for accessing templates and icons from a Salvo `Depot`.
- `StringExt` — convert a `String` into Salvo's HTML text responder.

A minimal rendering setup looks like this:

```rust
use sawe::templates::{Entry, Templates};

// `PageData` must implement serde::Serialize.
let mut templates = Templates::<&'static str>::new();
templates.add("site", Entry::new_static("./templates"))?;

let html = templates.render("site", "**/index.hbs", &page_data)?;
```

Place an `index.hbs` file under `./templates` and choose a glob that matches its asset path (for example, `**/index.hbs`). `page_data` is a serializable view model. Add `serde` with its `derive` feature to your application if you want to derive `Serialize` for your own view-model types.

The template API also supports rendering to a writer, rendering with a Handlebars `Context`, registering named templates, and looking up UI icons.

## SEO and structured data

The `templates` feature also exposes SEO-oriented helpers for composing page metadata:

- Base HTML metadata such as the page title, description, author, robots directives, canonical URL, icons, and web manifest.
- Open Graph metadata for link previews.
- Twitter card metadata.
- Schema.org JSON-LD structures, including graph entities and typed properties.
- Optional Bing and Google site-verification tags.

The JSON-LD type directory includes common Schema.org categories. Enable `google-type-dir` to include the additional Google-focused type definitions.

## OpenAPI documentation

With `docs` and `templates` enabled, `WebServer::add_oapi(...)` can associate an OpenAPI document with a router and mount that router at a chosen path. The server makes the registered specifications available through its documentation store, which can be accessed from Salvo's `Depot` using the documentation extension helpers.

This keeps route registration and API documentation connected without requiring you to build a separate OpenAPI router by hand.

## Public API at a glance

| API | Purpose |
| --- | --- |
| `sawe::server::WebServer` | Configure and run the HTTPS / HTTP/3 server. |
| `sawe::server::ServerError` | Errors returned while starting or running the server. |
| `sawe::salvo` | Salvo re-export. |
| `sawe::tokio` | Tokio re-export. |
| `sawe::templates::Entry` | Represent embedded or filesystem-backed template assets. |
| `sawe::templates::Templates<T>` | Register, find, and render templates and icons. |
| `sawe::templates::Seo` | Build HTML SEO metadata and verification tags. |
| `sawe::build_uiicons` | UI-icon build helper (requires `build` and `templates`). |

## Development

Clone the repository and check the code with your installed Rust toolchain:

```bash
git clone https://github.com/p1mo/sawe.git
cd sawe
git checkout dev
cargo check
```

To check the optional integrations together:

```bash
cargo check --all-features
cargo test --all-features
```

## License

`sawe` is dual-licensed. You may choose either:

- [MIT License](LICENSE_MIT)
- [Apache License, Version 2.0](LICENSE_APACHE-2.0)

---

<div align="center">

**Built on [Salvo](https://salvo.rs) · Published on [crates.io](https://crates.io/crates/sawe) · [API docs](https://docs.rs/sawe)**

</div>
