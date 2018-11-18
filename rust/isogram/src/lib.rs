use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    letter_map(candidate).values().all(|count| *count < 2)
}

fn letter_map(s: &str) -> HashMap<char, u32> {
    let mut letter_map = HashMap::new();
    for chr in s.chars() {
        if chr.is_ascii_alphabetic() {
            let lower = *chr.to_uppercase().collect::<Vec<_>>().get(0).unwrap();

            *letter_map.entry(lower).or_insert(0) += 1;
        }
    }
    letter_map
}
