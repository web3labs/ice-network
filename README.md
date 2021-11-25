# ICE Network
ICE Network is an EVM compatible network built with Parity’s Substrate framework.

## Setup

### Install Rust and Dependencies

Instal Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Nightly build (required for frontier)

```
rustup default nightly
```

Wasm toolchain

Depending on the OS, Wasm toolchain might require manual installation. 

E.g. on macOS (Big Sur v11.6), Wasm is added using following command

```
rustup target add wasm32-unknown-unknown --toolchain nightly-x86_64-apple-darwin
```
### Setup local frontier node

Clone pre-configure frontier template

```
git clone https://github.com/substrate-developer-hub/frontier-node-template/
```

Build

```
cd ./frontier-node-template
cargo build --release
```

Run the local dev node

```
./target/release/frontier-template-node --dev
```
