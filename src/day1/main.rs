use std::collections::HashMap;
use std::env;
use std::fs;


fn main() {
    let path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(path).unwrap();

    let word_digits = HashMap::from([
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'),
        ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'),
    ]);
    let mut total = 0u64;

    for line in input.lines() {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        let mut iter = line.chars();

        loop {
            let substr = iter.as_str();
            let next = iter.next();
            if next.is_none() {
                break;
            }

            let mut char = next.unwrap();

            for word in &word_digits {
                if substr.starts_with(word.0) {
                    char = *word.1;
                }
            }

            if char.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(char);
                    last_digit = Some(char);
                } else {
                    last_digit = Some(char);
                }
            }
        }

        let value = String::from_iter([first_digit.unwrap(), last_digit.unwrap()]);
        total += u64::from_str_radix(value.as_str(), 10).unwrap();

        println!("{} -> {}", line, value);
    }

    println!("total: {}", total);
}
