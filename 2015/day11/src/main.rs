// Solution for https://adventofcode.com/2015/day/11
use std::env;

fn valid(password: &str) -> bool {
    three_increasing(&password) && exclude_iol(&password) && has_two_pairs(&password)
}

fn invalid(password: &str) -> bool {
    !valid(password)
}

fn three_increasing(password: &str) -> bool {
    let mut count = 0;
    let mut last_char = None;
    for c in password.chars() {
        if last_char.is_none() {
            last_char = Some(c);
            count = 1;
            continue;
        }

        if let Some(prev_c) = last_char {
            if prev_c as u8 == c as u8 - 1 {
                count += 1;
            } else {
                count = 1;
            }
            last_char = Some(c);
        }

        if count >= 3 {
            return true;
        }
    }

    false
}

fn exclude_iol(password: &str) -> bool {
    !password.contains('i') && !password.contains('o') && !password.contains('l')
}

fn has_two_pairs(password: &str) -> bool {
    let mut matched_char: Option<char> = None;
    let mut matched_index: Option<usize> = None;
    for (index, ch) in password.chars().enumerate() {
        if ch == password.chars().nth(index + 1).unwrap_or('\0')
            && (matched_index.is_none() || matched_index.unwrap() + 1 < index)
        {
            if matched_char.is_none() {
                matched_char = Some(ch);
                matched_index = Some(index);
                continue;
            }

            if let Some(matched_char) = matched_char {
                if ch != matched_char {
                    return true;
                }
            }
        }
    }

    false
}

fn skip_iol(password: &mut String) {
    let len = password.len();
    let mut new_end = String::new();
    let mut index_from = 0;

    for (index, ch) in password.chars().enumerate() {
        if ch == 'i' || ch == 'o' || ch == 'l' {
            new_end = ((ch as u8 + 1) as char).to_string();
            new_end.push_str(&"a".repeat(len - index - 1));

            index_from = index;

            break;
        }
    }

    password.replace_range(index_from..len, &new_end);
}

fn increment(password: &mut String, len: usize) {
    // Optimization for skipping invalid characters i.e. (i, o and l)
    if !exclude_iol(password) {
        skip_iol(password);
        return;
    }

    // Base case: Empty password generates a new password with the first char.
    if len == 0 {
        password.insert(0, 'a');
        return;
    }

    // Recursive case, increment the last character.
    let c = password
        .chars()
        .nth(len - 1)
        .unwrap_or_else(|| panic!("No char at index {}", len - 1));

    let new_c = (c as u8 + 1) as char;

    // If an overflow is found, increment password[..-1] recursively.
    if c == 'z' {
        increment(password, len - 1);
        password.remove(len - 1);
        password.insert(len - 1, 'a');
    } else {
        password.remove(len - 1);
        password.insert(len - 1, new_c);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut password = args[1].to_owned();

    println!("Starting with password: {}", password);

    while invalid(&password) {
        let len = password.len();
        increment(&mut password, len);
    }

    let len = password.len();
    increment(&mut password, len);

    while invalid(&password) {
        let len = password.len();
        increment(&mut password, len);
    }

    println!("Next valid password is: {}", password);
}
