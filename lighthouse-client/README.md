# Lighthouse Client

[![crates.io](https://img.shields.io/crates/v/lighthouse-client)](https://crates.io/crates/lighthouse-client)
[![docs.rs](https://img.shields.io/docsrs/lighthouse-client)](https://docs.rs/lighthouse-client)

An asynchronous library for building clients for Project Lighthouse, e.g. games or animations.

The library is defined in terms of `futures` and can thus be used with any async runtime, though additional convenience functions are provided for `async-std` and `tokio`. The latter can be enabled via the corresponding crate features.

## Example Usage

First make sure to have a login at [lighthouse.uni-kiel.de](https://lighthouse.uni-kiel.de) and to have your credentials defined as environment variables:

```bash
export LIGHTHOUSE_USER=[your username]
export LIGHTHOUSE_TOKEN=[your api token]
```

You can now run an example with

```bash
cargo run --example disco
```

For a more complex example, try

```bash
cargo run --example snake
```
