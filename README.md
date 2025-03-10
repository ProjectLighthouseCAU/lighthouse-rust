# Project Lighthouse SDK for Rust

[![Build](https://github.com/ProjectLighthouseCAU/lighthouse-rust/actions/workflows/build.yml/badge.svg)](https://github.com/ProjectLighthouseCAU/lighthouse-rust/actions/workflows/build.yml)

Crates for interacting with Project Lighthouse in Rust, e.g. to build games or other clients.

| Crate | Version | Docs |
| - | - | - |
| [lighthouse-client](./lighthouse-client) | [![crates.io](https://img.shields.io/crates/v/lighthouse-client)](https://crates.io/crates/lighthouse-client) | [![docs.rs](https://img.shields.io/docsrs/lighthouse-client)](https://docs.rs/lighthouse-client) |
| [lighthouse-protocol](./lighthouse-protocol) | [![crates.io](https://img.shields.io/crates/v/lighthouse-protocol)](https://crates.io/crates/lighthouse-protocol) | [![docs.rs](https://img.shields.io/docsrs/lighthouse-protocol)](https://docs.rs/lighthouse-protocol) |

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

For more involved examples, check out

- https://github.com/ProjectLighthouseCAU/lighthouse-rust-template
- https://github.com/ProjectLighthouseCAU/limo
- https://github.com/fwcd/lisnake
- https://github.com/fwcd/litris
- https://github.com/fwcd/lidoom
