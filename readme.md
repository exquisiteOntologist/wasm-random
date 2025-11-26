# WASM Random

Simple WebAssembly random number generator.

## Why

There are a number of confusing random number libraries out there, and this one is designed to be simple and easy to use with WASM.

## Compatibility

This library does not depend on any system features, making it compatible with a wide range of environments including WASM.

## Usage

```rust
let random_number = wasm_random::random::random();
```

```rust
let random_number = wasm_random::random::random_more_precision()
```

```rust
let random_nmber = wasm_random::random::random_from_range(100., 300.);
```
