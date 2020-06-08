/*!
# Trim in-place

This crate is used for extending `String` in order to do in-place trimming.

## Usage

```rust
extern crate trim_in_place;

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
    fn trim_in_place(&mut self);
    fn trim_start_in_place(&mut self);
    fn trim_end_in_place(&mut self);

    // TODO trim_matches
}

impl TrimInPlace for String {
    #[inline]
    fn trim_in_place(&mut self) {
        let trimmed_str = self.trim();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }
    }

    #[inline]
    fn trim_start_in_place(&mut self) {
        let trimmed_str = self.trim_start();

        let trimmed_str_start_pointer = trimmed_str.as_ptr();
        let trimmed_str_length = trimmed_str.len();

        unsafe {
            let v = self.as_mut_vec();

            copy(trimmed_str_start_pointer, v.as_mut_ptr(), trimmed_str_length);

            v.set_len(trimmed_str_length);
        }
    }

    #[inline]
    fn trim_end_in_place(&mut self) {
        let trimmed_str_length = self.trim_end().len();

        unsafe {
            self.as_mut_vec().set_len(trimmed_str_length);
        }
    }
}
