# WASM Random

Simple WebAssembly random number generator.

## Why

A selection of random number libraries are available; however, few of those libraries are compatible with WASM. This library is designed to be simple and easy to use with WASM.

## Compatibility

This library does not depend on any system-specific features, making it compatible with a wide range of environments including WASM (`wasm32-unknown-unknown`).

## Usage

```rust
let random_number = wasm_random::random();
```

```rust
let random_number = wasm_random::random_from_range(100., 300.);
```

There are also `..._f64` variants of these functions (the default are `f32`).
