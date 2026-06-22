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

        v.set_len(len);
    }

    string.as_str()
}

#[inline]
fn set_len(string: &mut String, len: usize) -> &str {
    unsafe {
        // SAFETY: callers pass lengths produced by `str` trim methods, so they are always valid UTF-8 boundaries within the current string.
        string.as_mut_vec().set_len(len);
    }

    string.as_str()
}

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
    fn trim_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str;

    /// Trims matching text from the start of this string without allocating a new string.
    fn trim_start_matches_in_place<P: Pattern>(&mut self, pat: P) -> &str;

    /// Trims matching text from the end of this string without allocating a new string.
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
