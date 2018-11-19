use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let scores = create_score_map();
    word.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c|
            scores.get(&c.to_uppercase().nth(0).unwrap()).unwrap())
        .sum::<u64>()
}

fn  create_score_map() -> HashMap<char, u64> {
    let mut scores = HashMap::new();

    // score 1
    for c in vec!['A','E','I','O','U','L','N','R','S','T'] {
        scores.insert(c, 1);
    }

    // score 2
    for c in vec!['D','G'] {
        scores.insert(c, 2);
    }

    // score 3
    for c in vec!['B','C','M','P'] {
        scores.insert(c, 3);
    }

    // score 4
    for c in vec!['F','H','V','W','Y'] {
        scores.insert(c, 4);
    }

    // score 5
    scores.insert('K', 5);

    // score 8
    for c in vec!['J','X'] {
        scores.insert(c, 8);
    }

    // score 10
    for c in vec!['Q', 'Z'] {
        scores.insert(c, 10);
    }
    scores
}