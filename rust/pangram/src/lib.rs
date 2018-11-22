/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let chars = charset(sentence);
    count_unique(&chars) == 26
}

fn charset(sentence: &str) -> String {
    sentence.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_lowercase().nth(0).unwrap())
        .collect()
}

fn count_unique(charset: &str) -> usize {
    let mut chars = charset.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    chars.len()
}