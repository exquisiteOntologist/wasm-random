# WASM Random

Simple WebAssembly random number generator.

## Why

A selection of random number libraries are available; however, few of those libraries are compatible with WASM. This library is designed to be simple and easy to use with WASM.

## Compatibility

This library does not depend on any system-specific features, making it compatible with a wide range of environments including WASM (`wasm32-unknown-unknown`).

## Usage

### Random Numbers

```rust
let random_number = wasm_random::number::<u64>();
```

### Random in Range for Floats

```rust
let random_number = wasm_random::range(100., 300.);
```

### Random in Range for All Numeric Types

```rust
let random_number = wasm_random::range_all(130, 1500);
```

See the examples or read the source for further details.
