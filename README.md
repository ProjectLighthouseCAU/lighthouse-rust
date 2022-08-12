# Project Lighthouse Client for Rust

[![Build](https://github.com/fwcd/lighthouse-rust/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/lighthouse-rust/actions/workflows/build.yml)

An asynchronous API client for a light installation at the University of Kiel.

## Usage

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
