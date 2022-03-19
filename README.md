Trim in-place
====================

[![CI](https://github.com/magiclen/trim-in-place/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/trim-in-place/actions/workflows/ci.yml)

This crate is used for extending `String` in order to do in-place trimming.

## Usage

```rust
use trim_in_place::TrimInPlace;

let mut s = String::from(" 1234 abcd  ");

s.trim_in_place();

assert_eq!("1234 abcd", s);
```

## Benchmark

```bash
cargo bench
```

## Crates.io

https://crates.io/crates/trim-in-place

## Documentation

https://docs.rs/trim-in-place

## License

[MIT](LICENSE)