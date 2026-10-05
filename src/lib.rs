/*!
# Trim in-place

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
*/

#![no_std]

extern crate alloc;

mod pattern;

use alloc::string::String;
use core::ptr::copy;

pub use pattern::*;

#[inline]
fn move_to_front(string: &mut String, source: *const u8, len: usize) -> &str {
    unsafe {
        let v = string.as_mut_vec();

        // SAFETY: `source` and `len` come from a valid subslice of this string.
        // `ptr::copy` permits overlap, which is required for in-place trimming.
        copy(source, v.as_mut_ptr(), len);

        // SAFETY: the copied slice is valid UTF-8, and `len` is no larger than the original length.
        v.set_len(len);
    }

    string.as_str()
}

#[inline]
fn set_len(string: &mut String, len: usize) -> &str {
    unsafe {
        // SAFETY: callers pass the length of a valid UTF-8 prefix from `str` trim methods or sealed `Pattern` implementations.
        string.as_mut_vec().set_len(len);
    }

    string.as_str()
}

/// Trims strings in place.
///
/// The `String` implementation keeps its capacity and returns a slice of the updated string.
pub trait TrimInPlace {
    /// Trims Unicode whitespace from both ends of this string without allocating a new string.
    fn trim_in_place(&mut self) -> &str;

    /// Trims Unicode whitespace from the start of this string without allocating a new string.
    fn trim_start_in_place(&mut self) -> &str;

    /// Trims Unicode whitespace from the end of this string without allocating a new string.
    fn trim_end_in_place(&mut self) -> &str;

    /// Trims ASCII whitespace from both ends of this string without allocating a new string.
    fn trim_ascii_in_place(&mut self) -> &str;

    /// Trims ASCII whitespace from the start of this string without allocating a new string.
    fn trim_ascii_start_in_place(&mut self) -> &str;

    /// Trims ASCII whitespace from the end of this string without allocating a new string.
    fn trim_ascii_end_in_place(&mut self) -> &str;

    /// Trims matching text from both ends of this string without allocating a new string.
    /// For `&str` patterns, this removes repeated prefixes first and then repeated suffixes.
    /// An empty string pattern leaves the input unchanged.
    fn trim_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str;

    /// Trims matching text from the start of this string without allocating a new string.
    /// An empty string pattern leaves the input unchanged.
    fn trim_start_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str;

    /// Trims matching text from the end of this string without allocating a new string.
    /// An empty string pattern leaves the input unchanged.
    fn trim_end_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str;
}

impl TrimInPlace for String {
    #[inline]
    fn trim_in_place(&mut self) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = self.trim();

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_start_in_place(&mut self) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = self.trim_start();

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_end_in_place(&mut self) -> &str {
        let trimmed_str_length = self.trim_end().len();

        set_len(self, trimmed_str_length)
    }

    #[inline]
    fn trim_ascii_in_place(&mut self) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = self.trim_ascii();

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_ascii_start_in_place(&mut self) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = self.trim_ascii_start();

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_ascii_end_in_place(&mut self) -> &str {
        let trimmed_str_length = self.trim_ascii_end().len();

        set_len(self, trimmed_str_length)
    }

    #[inline]
    fn trim_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = pat.trim_matches_from(self);

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_start_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str {
        let (trimmed_str_start_pointer, trimmed_str_length) = {
            let trimmed_str = pat.trim_start_matches_from(self);

            (trimmed_str.as_ptr(), trimmed_str.len())
        };

        move_to_front(self, trimmed_str_start_pointer, trimmed_str_length)
    }

    #[inline]
    fn trim_end_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str {
        let trimmed_str_length = pat.trim_end_matches_from(self).len();

        set_len(self, trimmed_str_length)
    }
}
