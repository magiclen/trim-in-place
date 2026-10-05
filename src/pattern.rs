mod sealed {
    // This module keeps `Pattern` closed, so this crate controls every pattern behavior.
    pub trait Sealed {}

    impl Sealed for char {}
    impl Sealed for &str {}
    impl Sealed for &&str {}
    impl Sealed for &[char] {}

    impl<const N: usize> Sealed for [char; N] {}
    impl<const N: usize> Sealed for &[char; N] {}

    impl<F> Sealed for F where F: FnMut(char) -> bool {}
}

#[inline]
fn trim_str_matches<'a>(string: &'a str, pat: &str) -> &'a str {
    if pat.is_empty() {
        // An empty string matches everywhere, so trimming it must leave the input unchanged.
        return string;
    }

    let mut trimmed = string;

    while let Some(next) = trimmed.strip_prefix(pat) {
        trimmed = next;
    }

    while let Some(next) = trimmed.strip_suffix(pat) {
        trimmed = next;
    }

    trimmed
}

/// A stable pattern type that can be used by the in-place trim methods.
///
/// This trait is local to this crate because the standard library `Pattern` trait is still unstable to name in public APIs.
/// The supported patterns are `char`, `&str`, `&&str`, `&[char]`, character arrays and their references, and predicates like `|c| c == 'x'`.
/// This trait is sealed and cannot be implemented outside this crate.
pub trait Pattern: sealed::Sealed {
    /// Returns a subslice of `string` after trimming matching text from both ends.
    #[doc(hidden)]
    fn trim_matches_from(self, string: &str) -> &str;

    /// Returns a suffix of `string` after trimming matching text from the start.
    #[doc(hidden)]
    fn trim_start_matches_from(self, string: &str) -> &str;

    /// Returns a prefix of `string` after trimming matching text from the end.
    #[doc(hidden)]
    fn trim_end_matches_from(self, string: &str) -> &str;
}

impl Pattern for char {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        string.trim_matches(self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}

impl Pattern for &str {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        // Stable Rust can trim `&str` from one side, but not from both sides with `trim_matches`.
        trim_str_matches(string, self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}

impl Pattern for &&str {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        (*self).trim_matches_from(string)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        (*self).trim_start_matches_from(string)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        (*self).trim_end_matches_from(string)
    }
}

impl Pattern for &[char] {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        string.trim_matches(self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}

impl<const N: usize> Pattern for [char; N] {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        string.trim_matches(self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}

impl<const N: usize> Pattern for &[char; N] {
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        string.trim_matches(self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}

impl<F> Pattern for F
where
    F: FnMut(char) -> bool,
{
    #[inline]
    fn trim_matches_from(self, string: &str) -> &str {
        string.trim_matches(self)
    }

    #[inline]
    fn trim_start_matches_from(self, string: &str) -> &str {
        string.trim_start_matches(self)
    }

    #[inline]
    fn trim_end_matches_from(self, string: &str) -> &str {
        string.trim_end_matches(self)
    }
}
