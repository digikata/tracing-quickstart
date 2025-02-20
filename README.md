# Rust Tracing quickstart

The rust [tracing](https://docs.rs/tracing/latest/tracing/) and [tracing-subscriber](https://docs.rs/tracing/latest/tracing/)
crates are incredibly useful, but there aren't a lot of documents that go over
common ways of using tracing.

This repository contains a set of quickstart patterns for using the tracing system
configured to use a common setup that looks at the RUST_LOG env variable to
configure tracing outputs to stdout channel. By default, this project configures a default log
level of `trace`. (that part isn't common, usually off, or for servers info, warn is more typical)

You can read over the code and try out `cargo run` with various `RUST_LOG` settings
to see how the outputs change.

```
cargo run
RUST_LOG=warn cargo run
RUST_LOG=tracing_quickstart=off cargo run
RUST_LOG=info,tracing_quickstart=off cargo run
```
