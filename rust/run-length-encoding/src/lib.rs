extern crate regex;

use regex::Regex;

const ENCODED_PATTERNS: &str = r"(\d+\D)|\D";

const BASE_TEN_RADIX: u32 = 10;

pub fn encode(source: &str) -> String {
    let mut characters = source.chars();
    let mut last_char = characters.next().unwrap();
    let mut count = 1;
    let mut compressed = String::new();

    for c in characters {
        match c == last_char {
            true => {
                count += 1;
            }
            false => {
                match count {
                    1 => compressed.push_str(&format!("{}", last_char)),
                    _ => compressed.push_str(&format!("{}{}", count, last_char)),
                }
                last_char = c;
                count = 1;
            }
        }
    }

    // finally (last char will not be compared to anything)
    match count {
        1 => compressed.push_str(&format!("{}", last_char)),
        _ => compressed.push_str(&format!("{}{}", count, last_char)),
    }

    compressed
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();

    let patterns = extract_patterns(&source);

    for p in patterns {
        let digits = p
            .chars()
            .filter(|c| c.is_digit(BASE_TEN_RADIX))
            .collect::<String>();
        let characters = p
            .chars()
            .filter(|c| !c.is_digit(BASE_TEN_RADIX))
            .collect::<String>();
        match digits.len() {
            0 => decoded.push_str(&p), // simple pattern no repeating
            _ => {
                // repeat chars
                let repeats = digits.parse::<i32>().unwrap();
                for _ in 0..repeats {
                    decoded.push_str(&characters);
                }
            }
        }
    }

    decoded
}

fn extract_patterns(encoded: &str) -> Vec<String> {
    let re = Regex::new(ENCODED_PATTERNS).unwrap();
    let mut pattens: Vec<String> = Vec::new();
    for cap in re.captures_iter(encoded) {
        match cap.get(0) {
            Some(cap) => pattens.push(cap.as_str().into()),
            _ => {}
        }
    }

    pattens
}
