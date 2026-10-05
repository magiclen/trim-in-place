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

The methods update the original `String`, return a slice of the result, and keep its capacity.

### ASCII whitespace

ASCII trimming leaves non-ASCII whitespace unchanged.

```rust
use trim_in_place::TrimInPlace;

let mut s = String::from(" \t你好 \u{3000}");

assert_eq!("你好 \u{3000}", s.trim_ascii_in_place());
```

### Patterns

Patterns can be characters, string slices, character slices or arrays, and predicates.
The `Pattern` trait is sealed and cannot be implemented outside this crate.
String patterns remove repeated prefixes first, then repeated suffixes from the remaining text.
For example, trimming `"aba"` from `"ababa"` leaves `"ba"`.
An empty string pattern leaves the input unchanged.

```rust
use trim_in_place::TrimInPlace;

let mut s = String::from("abab1234 abcdab");

assert_eq!("1234 abcd", s.trim_matches_in_place("ab"));
```

### `no_std`

This crate supports `no_std` and requires `alloc` for `String`.

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
