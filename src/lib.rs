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

use core::intrinsics::copy;

use alloc::string::String;

pub trait TrimInPlace {
    fn trim_in_place(&mut self) -> &str;
    fn trim_start_in_place(&mut self) -> &str;
    fn trim_end_in_place(&mut self) -> &str;

    // TODO trim_matches with Pattern
    fn trim_matches_in_place(&mut self, pat: char) -> &str;
    fn trim_start_matches_in_place(&mut self, pat: char) -> &str;
    fn trim_end_matches_in_place(&mut self, pat: char) -> &str;
}

impl TrimInPlace for String {
    #[inline]
    fn trim_in_place(&mut self) -> &str {
        let trimmed_str = self.trim();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_start_in_place(&mut self) -> &str {
        let trimmed_str = self.trim_start();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_end_in_place(&mut self) -> &str {
        let trimmed_str_length = self.trim_end().len();

        unsafe {
            self.as_mut_vec().set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_matches_in_place(&mut self, pat: char) -> &str {
        let trimmed_str = self.trim_matches(pat);

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_start_matches_in_place(&mut self, pat: char) -> &str {
        let trimmed_str = self.trim_start_matches(pat);

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }

        self.as_str()
    }

    #[inline]
    fn trim_end_matches_in_place(&mut self, pat: char) -> &str {
        let trimmed_str_length = self.trim_end_matches(pat).len();

        unsafe {
            self.as_mut_vec().set_len(trimmed_str_length);
        }

        self.as_str()
    }
}
