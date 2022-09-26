# Project Lighthouse Client for Rust

[![Build](https://github.com/fwcd/lighthouse-rust/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/lighthouse-rust/actions/workflows/build.yml)

An asynchronous API client library for a light installation at the University of Kiel, written in Rust. The library is defined in terms of `futures` and can thus be used with any async runtime, though additional convenience functions are provided for `async-std` and `tokio`. The latter can be enabled via the corresponding crate features.

## Example Usage

First make sure to have a login at [lighthouse.uni-kiel.de](https://lighthouse.uni-kiel.de) and to have your credentials defined as environment variables:

```bash
export LIGHTHOUSE_USER=[your username]
export LIGHTHOUSE_TOKEN=[your api token]
```

You can now run an example with

```bash
cargo run --features async-std --example disco
```

For a more complex example, try

```bash
cargo run --features async-std --example snake
```

> **Note:** While the default tracing subscriber used by the examples doesn't log by default, setting the `RUST_LOG` environment variable e.g. to `info` will output the corresponding events.
