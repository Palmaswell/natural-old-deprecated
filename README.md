# Natural Animation Engine

## Getting started
1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Install the [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) cli installed.

```bash
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
3. Install the development watcher 

```bash
    cargo install cargo-watch
```

## Development
```bash
    # To kickstart the watcher you can run:
    cargo watch -s 'wasm-pack build --target web'
```

## Build

1. Build a wasm package manually.

```bash
    wasm-pack build --target web
```
