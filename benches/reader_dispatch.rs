#![feature(test)]

extern crate kg_io;
extern crate test;

use test::Bencher;

use kg_io::*;

fn make_string() -> String {
    let mut s = String::with_capacity((1024 * 1024 * 81) / 80);
    for i in 0..1024 * 1024 {
        let c: u8 = (i % (128 - 32)) + 32;
        s.push(c as char);
        if i % 80 == 0 {
            s.push('\n');
        }
    }
    s
}

fn dyn_scan(r: &mut dyn CharReader) -> ParseResult<usize> {
    let mut count = 0;
    while let Some(c) = r.next_char()? {
        if c == '0' {
            count += 1;
        }
    }
    Ok(count)
}

fn static_scan<R: CharReader>(r: &mut R) -> ParseResult<usize> {
    let mut count = 0;
    while let Some(c) = r.next_char()? {
        if c == '0' {
            count += 1;
        }
    }
    Ok(count)
}


#[bench]
fn dyn_dispatch(b: &mut Bencher) {
    let s = make_string();
    let mut reader = MemCharReader::new(s.as_bytes());

    b.iter(|| dyn_scan(&mut reader));
}

#[bench]
fn static_dispatch(b: &mut Bencher) {
    let s = make_string();
    let mut reader = MemCharReader::new(s.as_bytes());

    b.iter(|| static_scan(&mut reader));
}