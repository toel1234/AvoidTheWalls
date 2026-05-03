# Avoid The Walls (Bevy Edition)

A modern Rust port of the "Avoid The Walls" game, originally built in Unity.

## Prerequisites

### Linux
To build on Linux, you need the following development libraries:
- `libasound2-dev` (ALSA)
- `libudev-dev` (udev)

On Ubuntu/Debian:
```bash
sudo apt-get install libasound2-dev libudev-dev
```

### macOS / Windows
Standard Rust installation should be sufficient.

## Building and Running

### Native
```bash
cargo run --release
```

### Web (WASM)
To build for the web:
1. Install the wasm32 target: `rustup target add wasm32-unknown-unknown`
2. Install `wasm-server-runner`: `cargo install wasm-server-runner`
3. Run: `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner cargo run --target wasm32-unknown-unknown`

## Features
- **Modern ECS Architecture:** Built with the Bevy game engine.
- **Cross-Platform:** Supports Desktop, Web, and Mobile.
- **Physics-Based:** Uses `bevy_rapier2d` for precise collisions.
- **Procedural Level Generation:** Endless runner logic translated from the original Unity scripts.
- **High Performance:** Native Rust execution with minimal overhead.
