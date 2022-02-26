# Install:

## Wasm & cargo-watch

```sh
# Install Wasm
rustup target install wasm32-unknown-unknown
# Runs our Wasm game in a browser
cargo install wasm-server-runner
# Automatically rebuilds our game on save
cargo install cargo-watch
```

# Build:

```sh
cargo run --target wasm32-unknown-unknown # Normally this binary is too large
cargo run --release --target wasm32-unknown-unknown # So we use release instead
```

# Run

Make sure you've setup .cargo/config.toml

```sh
cargo watch -cx "run --release"
```