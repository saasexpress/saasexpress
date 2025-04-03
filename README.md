## Development

https://dev.to/jorgecastro/hot-reload-in-rust-with-cargo-watch-5bon

> NOTE: Added `"rust-analyzer.linkedProjects": ["rust-example/Cargo.toml"]` to `settings.json`

```sh
cargo watch -w src -x run
```

http://127.0.0.1:8080/static/index.html

## Releasing

```sh
cargo build --release
```

This will produce an executable that can be run:

```sh
target/release/cargo-watch-example
```
