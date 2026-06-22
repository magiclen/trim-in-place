use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use trim_in_place::*;

const TEXT: &str = " 1234 abcd  ";

#[inline]
fn reset(input: &mut String) {
    input.clear();
    input.push_str(black_box(TEXT));
}

#[inline]
fn consume(output: &str) {
    let checksum =
        output.as_bytes().iter().fold(0u8, |checksum, byte| checksum.wrapping_add(*byte));

    black_box((output.len(), checksum));
}

fn trim(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        let output = input.trim().to_string();

        consume(&output);
    });
}

fn trim_in_place(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        input.trim_in_place();
        consume(input.as_str());
    });
}

fn trim_start(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        let output = input.trim_start().to_string();

        consume(&output);
    });
}

fn trim_start_in_place(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        input.trim_start_in_place();
        consume(input.as_str());
    });
}

fn trim_end(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        let output = input.trim_end().to_string();

        consume(&output);
    });
}

fn trim_end_in_place(bencher: &mut Bencher) {
    bencher.bytes = TEXT.len() as u64;

    let mut input = String::with_capacity(TEXT.len());

    bencher.iter(|| {
        reset(&mut input);

        input.trim_end_in_place();
        consume(input.as_str());
    });
}

benchmark_group!(_trim, trim, trim_in_place);
benchmark_group!(_trim_start, trim_start, trim_start_in_place);
benchmark_group!(_trim_end, trim_end, trim_end_in_place);

benchmark_main!(_trim, _trim_start, _trim_end);
