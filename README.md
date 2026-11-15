<div align="center">

# sawe

**A Salvo toolkit for serving modern web applications, rendering templates, and generating structured metadata.**

[![Crates.io](https://img.shields.io/crates/v/sawe?style=for-the-badge)](https://crates.io/crates/sawe)
[![Documentation](https://img.shields.io/docsrs/sawe?style=for-the-badge)](https://docs.rs/sawe)
[![License](https://img.shields.io/crates/l/sawe?style=for-the-badge)](LICENSE_MIT)

**HTTP/3 · TLS · Handlebars · UFS · UIICONS · SEO · Schema.org JSON-LD · OpenAPI**

</div>

---

## Overview

`sawe` is a Rust crate built around the [Salvo](https://salvo.rs) web framework. It provides a small server wrapper for TCP and QUIC listeners, plus optional modules for template and asset handling, UI icons, SEO metadata, Schema.org JSON-LD, and OpenAPI documentation.

The crate is feature-gated: the server is available with the default configuration, while templates, documentation, and SEO can be enabled independently. The crate uses the Rust 2024 edition, so use a toolchain that supports it.

## Features

- **Modern web serving** — serve TCP traffic (HTTP/1 and HTTP/2) and QUIC traffic (HTTP/3) on the configured address.
- **TLS setup** — load PEM certificate/key files or generate a local self-signed certificate when neither exists.
- **Graceful shutdown** — react to Ctrl+C and termination signals, with an optional graceful-shutdown timeout.
- **Handlebars templates** — find and render templates from filesystem-backed or embedded UFS assets.
- **UI icons** — access embedded icons and perform exact or partial icon lookups.
- **SEO metadata** — generate standard HTML metadata, Open Graph tags, Twitter cards, and site-verification tags.
- **Schema.org JSON-LD** — build graph entities from a typed directory of Schema.org types and properties.
- **OpenAPI documentation** — attach OpenAPI routers to the server and render documentation using an embedded Handlebars layout.
- **Salvo and Tokio re-exports** — use the same framework/runtime versions selected by the crate.

## Installation

Add `sawe` from [crates.io](https://crates.io/crates/sawe):

```toml
[dependencies]
sawe = "0.1.1"
```

To enable the complete feature set, use the `all` feature:

```toml
[dependencies]
sawe = { version = "0.1.1", features = ["all"] }
```

## Feature flags

The default feature set is empty (`default = []`). Enable only the modules your application needs.

| Feature | API / functionality |
| --- | --- |
| *(no feature)* | Core `WebServer`, `ServerError`, `salvo` and `tokio` re-exports. |
| `templates` | `sawe::templates`: Handlebars rendering, UFS-backed entries, UI-icon access, and Depot extension methods. |
| `docs` | OpenAPI router integration, `DocsStore`, `DocsDepotExt`, and the embedded documentation layout. Enable `templates` as well because the docs implementation uses Handlebars. |
| `templates-build` | Enables the crate-root `sawe::build_uiicons` build-helper re-export. |
| `seo` | `sawe::seo`: HTML metadata helpers and Schema.org JSON-LD types. |
| `seo-google` | Adds Google-focused JSON-LD type variants; use together with `seo`. |
| `all` | Enables `docs`, `templates`, `templates-build`, `seo`, and `seo-google`. |

### Common configurations

Server only:

```toml
sawe = "0.1.1"
```

Templates and icons:

```toml
sawe = { version = "0.1.1", features = ["templates"] }
```

OpenAPI documentation (with its Handlebars dependency):

```toml
sawe = { version = "0.1.1", features = ["docs", "templates"] }
```

SEO and Schema.org types, including the Google-specific directory:

```toml
sawe = { version = "0.1.1", features = ["seo", "seo-google"] }
```

Everything:

```toml
sawe = { version = "0.1.1", features = ["all"] }
```

## Quick start: run the server

`WebServer` accepts a Salvo router, a catcher, an optional shutdown timeout, a bind address, and a path for TLS files.

```rust
use sawe::{
    salvo::{catcher::Catcher, prelude::Router},
    tokio,
    WebServer,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sawe::ServerError> {
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

Add your application routes to the `Router` before passing it to `WebServer::new`.

### TLS files and HTTPS

At startup, the server looks for these files inside the TLS directory:

```text
.sawe/tls/
├── certs.pem
└── key.pem
```

If **both files are absent**, `sawe` creates the directory and generates a self-signed certificate. The default subject alternative names are `localhost`, `127.0.0.1`, `0.0.0.0`, `::1`, and `::`.

If the files already exist, the server reads them as the certificate chain and private key. Provide both files; a partial pair cannot be loaded.

The server installs Salvo's `ForceHttps` hoop using the configured port. For the example above, browse to `https://127.0.0.1:5800`. A browser warning is expected when using the generated self-signed certificate.

**Production:** supply a certificate trusted by your clients and its matching private key. The generated certificate is intended for local development, not public production traffic. Keep `key.pem` out of version control.

## Templates and assets

Enable `templates` to use `sawe::templates`.

### Register an entry and render a template

Entries can be backed by a directory on disk or by an embedded `ufs::UnifiedFS`.

```rust
use sawe::templates::{Entry, Templates};

#[derive(serde::Serialize)]
struct PageData {
    title: String,
}

fn render_page() -> Result<String, Box<dyn std::error::Error>> {
    let mut templates = Templates::<&'static str>::new();
    templates.add("site", Entry::new_static("./templates"))?;

    let data = PageData {
        title: "Hello from sawe".to_string(),
    };

    let html = templates.render("site", "**/index.hbs", &data)?;
    Ok(html)
}
```

Place an `index.hbs` file somewhere under `./templates`. The `search` argument is a `globset` pattern matched against asset paths; `**/index.hbs` matches an `index.hbs` file at any depth.

The example uses Serde's derive macro. Add `serde = { version = "1", features = ["derive"] }` to your application's dependencies when deriving `Serialize` for your own data types.

### Entry sources

| Constructor | Description |
| --- | --- |
| `Entry::new_static(path)` | Walk a directory on disk when looking up an asset. |
| `Entry::new_embed(fs)` | Look up assets from an existing `ufs::UnifiedFS`. |

### Template API

`Templates<T>` is keyed by an application-defined value `T` (`PartialEq + Send + Sync`).

| Method | Purpose |
| --- | --- |
| `Templates::new()` | Create an empty template registry. |
| `Templates::new_with_icons(key, icons)` | Create a registry with an embedded icon collection for a key. |
| `add(key, entry)` | Register an entry; returns an error if the key already exists. |
| `get_entry(key)` | Get a registered `Entry`. |
| `find(key, pattern)` | Find matching asset bytes. |
| `render(key, pattern, data)` | Render a matching Handlebars template from any `Serialize` value. |
| `render_to_write(key, pattern, data, writer)` | Render a template directly into a writer. |
| `render_with_context(key, pattern, context)` | Render with a Handlebars `Context`. |
| `render_with_context_to_write(key, pattern, context, writer)` | Render a context directly into a writer. |
| `register_template(key, pattern, name)` | Register a matching asset under a Handlebars template name. |
| `render_template(name, value)` | Render a previously registered template using a `serde_json::Value`. |
| `icon_find(key, search)` | Find an embedded icon by its exact lookup. |
| `icon_par_find(key, search)` | Find an embedded icon using the partial lookup. |
| `to_rw()` | Wrap the registry in `Arc<parking_lot::RwLock<_>>` for shared access. |

The module also exposes:

- `RwTemplates<T>` — the shared `Arc<RwLock<Templates<T>>>` type.
- `EmbededIcons` and `embeded_icons` — UI-icon types/macros re-exported from `uiicons` (the spellings are part of the current API).
- `build_uiicons` — the UI-icon build helper re-exported from the templates module.
- `TemplatesDepotExt` — extension methods for using templates and icons from a Salvo `Depot`.
- `StringExt::to_html()` — converts a `String` into Salvo's HTML `Text` responder.
- `ufs` — the UFS crate re-export.

`TemplatesDepotExt` provides `icons_find`, `icons_par_find`, `tpl_find`, `tpl_render`, `tpl_render_to_write`, `tpl_render_with_context`, and `tpl_render_with_context_to_write`. The corresponding `RwTemplates<T>` value must be present in the request `Depot` for these helpers to work.

## OpenAPI documentation

Enable both `docs` and `templates` to use the documentation integration. `WebServer::add_oapi` accepts a title, version, route path, and Salvo router. It derives an OpenAPI document from that router and mounts the generated documentation route alongside it.

```rust
use sawe::{
    salvo::{catcher::Catcher, prelude::Router},
    WebServer,
};
use std::time::Duration;

let router = Router::new();
let api_router = Router::new();

// Add your API endpoints to `api_router` before registering it.

let mut server = WebServer::new(
    router,
    Catcher::default(),
    Some(Duration::from_secs(10)),
    ("127.0.0.1", 5800),
    "./.sawe/tls",
);

server.add_oapi("Example API", "1.0.0", "/api/docs", api_router);

// In an async context:
// server.run().await?;
```

The docs feature provides:

- `DocsStore` — stores the OpenAPI documents registered with the server.
- `ArcDocsStore` — shared, lock-protected documentation-store type.
- `DocsDepotExt` — `Depot` methods `docs_find(path)` and `docs_render(path, template)`.
- `DEFAULT_DOCS_FILE` — the embedded Handlebars layout name, currently `documentation.hbs`.
- `DocsError` — errors for missing documents, template rendering, serialization, and related operations.

The documentation layout is embedded from `layouts/documentation.hbs`. `docs_render` renders a registered OpenAPI document with the selected embedded template.

## SEO and structured data

Enable `seo` to use `sawe::seo`.

### HTML metadata

The SEO module provides:

- `BaseEntity` for standard document metadata such as title, description, author, robots directives, Googlebot directives, application name, theme color, canonical URL, favicon, Apple touch icon, and web manifest.
- `OpenGraphEntity` for Open Graph tags.
- `TwitterEntity` for Twitter card tags.
- `Seo` for holding and rendering these entities and optional Bing / Google site-verification tokens.

`Seo::new(bing_token, google_token)` creates the SEO helper. Use `set_base_entity`, `set_og_entity`, and `set_twitter_entity` to assign metadata entities, then call `meta_to_html()` to produce the corresponding markup.

The entity structs derive Serde serialization and deserialization. Their fields are private in the current API, so populate custom values by deserializing them from a Serde-supported format (for example, JSON) rather than by assigning fields directly. This example uses `serde_json`, which your application must declare as a direct dependency:

```rust
use sawe::seo::{BaseEntity, Seo};

fn render_head() -> Result<String, Box<dyn std::error::Error>> {
    let base: BaseEntity = serde_json::from_str(
        r#"{
            "title": "Example",
            "description": "An example website",
            "canonical": "https://example.com/"
        }"#,
    )?;

    let seo = Seo::new(None, None);
    seo.set_base_entity(base);

    Ok(seo.meta_to_html())
}
```

### Schema.org JSON-LD

The JSON-LD API is available through the same `seo` feature. It includes:

- `JsonLD` — the top-level JSON-LD document (`@context`, `@graph`, `@id`, and `@type`).
- `GraphEntity` — a strongly typed entity with a broad set of Schema.org properties (the source identifies its vocabulary as Schema.org 30.1).
- `TypeDirectory` — grouped enums for common types such as websites, organizations, local businesses, people, places, products, offers, reviews, articles, media, education, events, software, services, and more.
- `Custom(String)` — a custom type name when a built-in variant does not fit.

Example:

```rust
use sawe::seo::{GraphEntity, JsonLD, TypeDirectory, Websites};

fn json_ld_script() -> Result<String, sawe::Error> {
    let website = GraphEntity {
        id: Some("https://example.com/#website".to_string()),
        typedir: TypeDirectory::Websites(Websites::WebSite),
        name: Some("Example".to_string()),
        url: Some("https://example.com/".to_string()),
        ..Default::default()
    };

    let json_ld = JsonLD::new_graph(
        vec![website],
        "https://example.com/#structured-data",
        TypeDirectory::Websites(Websites::WebSite),
    );

    json_ld.to_string_pretty(true)
}
```

`to_string(false)` and `to_string_pretty(false)` return JSON text. Pass `true` to wrap the JSON in an `application/ld+json` script element.

Enable `seo-google` together with `seo` to expose the additional Google-oriented type enums, such as Google article, breadcrumb, carousel, event, product, recipe, review, software, video, and other rich-result types.

> **Current API note:** `Seo::meta_to_html_with_jsonld()` currently returns the same metadata markup as `meta_to_html()`; it does not append the JSON-LD script. Generate JSON-LD separately with `JsonLD::to_string(true)` and include the result in your document head.

## Public API map

| Path | Main exports |
| --- | --- |
| `sawe` | `WebServer`, `ServerError`, `Error`, `salvo`, `tokio`; feature-gated SEO and template re-exports. |
| `sawe::server` | Server implementation and, with `docs`, `DocsStore`, `DocsDepotExt`, `DocsError`, and related types. |
| `sawe::templates` | `Entry`, `Templates<T>`, `RwTemplates<T>`, icon types, Depot extensions, and `ufs`. |
| `sawe::seo` | `Seo`, metadata entities, `JsonLD`, `GraphEntity`, and `TypeDirectory` (requires `seo`). |
| `sawe::build_uiicons` | UI-icon build helper (crate-root re-export requires `templates-build`). |

## Development

Clone the repository to work with the source code:

```bash
git clone https://github.com/p1mo/sawe.git
cd sawe
```

Check the default build and the complete feature set:

```bash
cargo check
cargo check --all-features
cargo test --all-features
cargo fmt -- --check
```

## License

`sawe` is dual-licensed. You may choose either:

- [MIT License](LICENSE_MIT)
- [Apache License, Version 2.0](LICENSE_APACHE-2.0)

---

<div align="center">

**Built on [Salvo](https://salvo.rs) · [crates.io](https://crates.io/crates/sawe) · [API documentation](https://docs.rs/sawe)**

</div>
