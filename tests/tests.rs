use trim_in_place::TrimInPlace;

#[test]
fn trim() {
    {
        let mut s = String::from("");

        s.trim_in_place();

        assert_eq!("", s);
    }

    {
        let mut s = String::from(" 1234 abcd  ");

        s.trim_in_place();

        assert_eq!("1234 abcd", s);
    }

    {
        let mut s = String::from("\u{3000}你好\u{2009}");

        s.trim_in_place();

        assert_eq!("你好", s);
    }
}

#[test]
fn trim_start() {
    {
        let mut s = String::from("");

        s.trim_start_in_place();

        assert_eq!("", s);
    }

    {
        let mut s = String::from(" 1234 abcd  ");

        s.trim_start_in_place();

        assert_eq!("1234 abcd  ", s);
    }

    {
        let mut s = String::from("\u{3000}你好\u{2009}");

        s.trim_start_in_place();

        assert_eq!("你好\u{2009}", s);
    }
}

#[test]
fn trim_end() {
    {
        let mut s = String::from("");

        s.trim_end_in_place();

        assert_eq!("", s);
    }

    {
        let mut s = String::from(" 1234 abcd  ");

        s.trim_end_in_place();

        assert_eq!(" 1234 abcd", s);
    }

    {
        let mut s = String::from("\u{3000}你好\u{2009}");

        s.trim_end_in_place();

        assert_eq!("\u{3000}你好", s);
    }
}

#[test]
fn trim_matches() {
    {
        let mut s = String::from("");

        s.trim_matches_in_place('X');

        assert_eq!("", s);
    }

    {
        let mut s = String::from("X1234 abcdXX");

        s.trim_matches_in_place('X');

        assert_eq!("1234 abcd", s);
    }

    {
        let mut s = String::from("X你好XX");

        s.trim_matches_in_place('X');

        assert_eq!("你好", s);
    }
}

#[test]
fn trim_start_matches() {
    {
        let mut s = String::from("");

        s.trim_start_matches_in_place('X');

        assert_eq!("", s);
    }

    {
        let mut s = String::from("X1234 abcdXX");

        s.trim_start_matches_in_place('X');

        assert_eq!("1234 abcdXX", s);
    }

    {
        let mut s = String::from("X你好XX");

        s.trim_start_matches_in_place('X');

        assert_eq!("你好XX", s);
    }
}

#[test]
fn trim_end_matches() {
    {
        let mut s = String::from("");

        s.trim_end_matches_in_place('X');

        assert_eq!("", s);
    }

    {
        let mut s = String::from("X1234 abcdXX");

        s.trim_end_matches_in_place('X');

        assert_eq!("X1234 abcd", s);
    }

    {
        let mut s = String::from("X你好XX");

        s.trim_end_matches_in_place('X');

        assert_eq!("X你好", s);
    }
}

#[test]
fn trim_matches_str_pattern() {
    let mut s = String::from("abab1234 abcdab");

    s.trim_matches_in_place("ab");

    assert_eq!("1234 abcd", s);
}

#[test]
fn trim_matches_str_ref_pattern() {
    let mut s = String::from("abab1234 abcdab");
    let pat: &&str = &"ab";

    s.trim_matches_in_place(pat);

    assert_eq!("1234 abcd", s);
}

#[test]
fn trim_start_matches_str_pattern() {
    let mut s = String::from("abab1234 abcdab");

    s.trim_start_matches_in_place("ab");

    assert_eq!("1234 abcdab", s);
}

#[test]
fn trim_end_matches_str_pattern() {
    let mut s = String::from("ab1234 abcdabab");

    s.trim_end_matches_in_place("ab");

    assert_eq!("ab1234 abcd", s);
}

#[test]
fn trim_matches_empty_str_pattern() {
    {
        let mut s = String::from("1234 abcd");

        s.trim_matches_in_place("");

        assert_eq!("1234 abcd", s);
    }

    {
        let mut s = String::from("1234 abcd");

        s.trim_start_matches_in_place("");

        assert_eq!("1234 abcd", s);
    }

    {
        let mut s = String::from("1234 abcd");

        s.trim_end_matches_in_place("");

        assert_eq!("1234 abcd", s);
    }
}

#[test]
fn trim_matches_char_slice_pattern() {
    let mut s = String::from("XY1234 abcdYX");
    let pat: &[char] = &['X', 'Y'];

    s.trim_matches_in_place(pat);

    assert_eq!("1234 abcd", s);
}

#[test]
fn trim_start_matches_char_array_pattern() {
    let mut s = String::from("XY1234 abcdYX");

    s.trim_start_matches_in_place(['X', 'Y']);

    assert_eq!("1234 abcdYX", s);
}

#[test]
fn trim_end_matches_char_array_ref_pattern() {
    let mut s = String::from("XY1234 abcdYX");
    let pat: &[char; 2] = &['X', 'Y'];

    s.trim_end_matches_in_place(pat);

    assert_eq!("XY1234 abcd", s);
}

#[test]
fn trim_matches_predicate_pattern() {
    let mut s = String::from("1234 abcd  ");

    s.trim_matches_in_place(|c: char| c.is_ascii_digit() || c.is_ascii_whitespace());

    assert_eq!("abcd", s);
}

#[test]
fn trim_matches_unicode_pattern() {
    let mut s = String::from("界界1234 abcd界");

    s.trim_matches_in_place('界');

    assert_eq!("1234 abcd", s);
}

#[test]
fn trim_matches_returns_result_and_keeps_capacity() {
    let mut s = String::with_capacity(64);

    s.push_str("abab1234 abcdab");

    let capacity = s.capacity();

    assert_eq!("1234 abcd", s.trim_matches_in_place("ab"));
    assert_eq!("1234 abcd", s);
    assert_eq!(capacity, s.capacity());
}
