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
fn trim_ascii() {
    {
        let mut s = String::from("");

        s.trim_ascii_in_place();

        assert_eq!("", s);
    }

    {
        let mut s = String::from(" \t\n1234 abcd\r\n ");

        s.trim_ascii_in_place();

        assert_eq!("1234 abcd", s);
    }

    {
        let mut s = String::from("\u{3000} 你好 \u{2009}");

        s.trim_ascii_in_place();

        assert_eq!("\u{3000} 你好 \u{2009}", s);
    }
}

#[test]
fn trim_ascii_start() {
    {
        let mut s = String::from(" \t\n1234 abcd\r\n ");

        s.trim_ascii_start_in_place();

        assert_eq!("1234 abcd\r\n ", s);
    }

    {
        let mut s = String::from("\u{3000} 你好 ");

        s.trim_ascii_start_in_place();

        assert_eq!("\u{3000} 你好 ", s);
    }
}

#[test]
fn trim_ascii_end() {
    {
        let mut s = String::from(" \t\n1234 abcd\r\n ");

        s.trim_ascii_end_in_place();

        assert_eq!(" \t\n1234 abcd", s);
    }

    {
        let mut s = String::from(" 你好 \u{2009}");

        s.trim_ascii_end_in_place();

        assert_eq!(" 你好 \u{2009}", s);
    }
}

#[test]
fn trim_ascii_returns_result_and_keeps_capacity() {
    let mut s = String::with_capacity(64);

    s.push_str(" \t1234 abcd\n ");

    let capacity = s.capacity();

    assert_eq!("1234 abcd", s.trim_ascii_in_place());
    assert_eq!("1234 abcd", s);
    assert_eq!(capacity, s.capacity());
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
