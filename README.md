# sawe

<p>
  <a href="https://crates.io/crates/sawe">
    <img src="https://img.shields.io/crates/v/sawe?style=flat-square" alt="Crates.io version">
  </a>
</p>

A lightweight Rust toolkit built around [Salvo](https://salvo.rs), bringing together HTTP/3 serving, Handlebars templates, UFS assets, UI icons, and SEO helpers.

## Install

```sh
cargo add sawe --features templates
```

## Features

- Salvo HTTP/3 server with TLS and graceful shutdown.
- Handlebars rendering from embedded or static UFS files (`templates` feature).
- UI icon lookup and build integration.
- SEO metadata helpers for standard, Open Graph, and Twitter tags.

Enable `build` for the UI-icons build helper, or `google-type-dir` for additional schema type definitions.

## License

Licensed under either [MIT](LICENSE_MIT) or [Apache-2.0](LICENSE_APACHE-2.0), at your option.
