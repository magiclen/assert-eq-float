assert-eq-float
====================

[![CI](https://github.com/magiclen/assert-eq-float/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/assert-eq-float/actions/workflows/ci.yml)

This crate provides the `assert_eq_float!` macro that supports floats.

## Examples

```rust
use assert_eq_float::assert_eq_float;

assert_eq_float!(1.1 + 0.1, 1.2); // error = 0.0000000000000021316282072803005
assert_eq_float!(1e100 + 2e100, 3e100); // error = 53290705182007510000000000000000000000000000000000000000000000000000000000000000000000
```

The default margin of error is dynamically computed by properties of IEEE 754 floating point numbers. You don't need to worry about it if you just want to check two float values are **equal**.

## Crates.io

https://crates.io/crates/assert-eq-float

## Documentation

https://docs.rs/assert-eq-float

## License

[MIT](LICENSE)