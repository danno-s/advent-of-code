// Solution for https://adventofcode.com/2015/day/10
use std::env;

fn look_and_see(value: &str) -> String {
    let mut count = 0;
    let mut curr_char: Option<char> = None;
    let mut out = "".to_owned();

    for c in value.chars() {
        if curr_char.is_none() {
            curr_char = Some(c);
            count = 1;
            continue;
        }

        if let Some(some_char) = curr_char {
            if some_char != c {
                out.push_str(count.to_string().as_str());
                out.push(some_char);
                curr_char = Some(c);
                count = 1;
                continue;
            }

            count += 1;
        }
    }

    out.push_str(count.to_string().as_str());
    out.push(curr_char.unwrap());

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut value: String = args[1].to_owned();

    println!("Starting with: {}", value);

    for i in 0..50 {
        value = look_and_see(&value);
    }

    println!("Result after 40 iterations has {} characters.", value.len());
}
