// Solution for https://adventofcode.com/2015/day/8
use regex::Regex;
use std::fs;

#[macro_use]
extern crate lazy_static;

fn count_decode_diff(line: &str, diff: &mut usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(?P<backslash>\\\\)|(?P<doublequote>\\")|(?P<hex>\\x[\dA-Fa-f]{2})"#)
                .unwrap();
    }

    // Start counting the first and last doublequotes
    *diff += 2;
    for captures in RE.captures_iter(line) {
        if let Some(m) = captures.name("backslash") {
            *diff += 1;
        } else if let Some(m) = captures.name("doublequote") {
            *diff += 1;
        } else if let Some(m) = captures.name("hex") {
            *diff += 3;
        }
    }
}

fn count_encode_diff(line: &str, diff: &mut usize) {
    // Start counting the first and last doublequotes
    *diff += 2;

    *diff += line.matches('\\').count() + line.matches('"').count();
}

fn main() {
    let filename = "input/input.txt";

    let contents = fs::read_to_string(filename).unwrap();

    let mut decode: usize = 0;
    let mut encode: usize = 0;

    for line in contents.lines() {
        count_decode_diff(line, &mut decode);
        count_encode_diff(line, &mut encode);
    }

    println!("Difference in memory if decoding: {:?}", decode);
    println!("Difference in memory if encoding: {:?}", encode);
}
