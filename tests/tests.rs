extern crate trim_in_place;

use trim_in_place::TrimInPlace;

#[test]
fn trim() {
    let mut s = String::from(" 1234 abcd  ");

    s.trim_in_place();

    assert_eq!("1234 abcd", s);
}

#[test]
fn trim_start() {
    let mut s = String::from(" 1234 abcd  ");

    s.trim_start_in_place();

    assert_eq!("1234 abcd  ", s);
}

#[test]
fn trim_end() {
    let mut s = String::from(" 1234 abcd  ");

    s.trim_end_in_place();

    assert_eq!(" 1234 abcd", s);
}
