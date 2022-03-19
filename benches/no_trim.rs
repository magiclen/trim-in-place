use bencher::{benchmark_group, benchmark_main, Bencher};

use trim_in_place::*;

const TEXT: &str = "1234 abcd";

fn trim(bencher: &mut Bencher) {
    bencher.iter(|| {
        let s = String::from(TEXT);

        s.trim().to_string()
    });
}

fn trim_in_place(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut s = String::from(TEXT);

        s.trim_in_place();

        s
    });
}

fn trim_start(bencher: &mut Bencher) {
    bencher.iter(|| {
        let s = String::from(TEXT);

        s.trim_start().to_string()
    });
}

fn trim_start_in_place(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut s = String::from(TEXT);

        s.trim_start_in_place();

        s
    });
}

fn trim_end(bencher: &mut Bencher) {
    bencher.iter(|| {
        let s = String::from(TEXT);

        s.trim_end().to_string()
    });
}

fn trim_end_in_place(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut s = String::from(TEXT);

        s.trim_end_in_place();

        s
    });
}

benchmark_group!(_trim, trim, trim_in_place);
benchmark_group!(_trim_start, trim_start, trim_start_in_place);
benchmark_group!(_trim_end, trim_end, trim_end_in_place);

benchmark_main!(_trim, _trim_start, _trim_end);
