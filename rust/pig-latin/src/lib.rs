

const VOWELS: [char; 5] = ['a', 'e', 'u', 'i', 'o'];

pub fn translate(input: &str) -> String {
    match first_is_vowel(&input) {
        true => return vowel_ay(&input),
        false => {
            match has_qu(&input) {
                true => {
                    println!("hasqu alright!!");
                    return consonant_qu_swap(&input)
                },
                false => return consonant_swap(&input)
            }
        }
    }

    input.into()
}

fn vowel_ay(input: &str) -> String {
    format!("{}{}", input, "ay")
}

fn consonant_swap(input: &str) -> String {
    format!("{}{}ay", skip_first(input), get_first_unsafe(input))
}

fn consonant_qu_swap(input: &str) -> String {
    format!("{}{}quay", input.chars().skip(3).collect::<String>(), get_first_unsafe(input))

}

fn first_is_vowel(input: &str) -> bool {
    input.chars().take(1).any(|x| VOWELS.contains(&x))
        || ["yt".into(), "xr".into()]
            .contains(&input.chars().take(2).collect::<String>())
}

fn get_first_unsafe(input: &str) -> String {
    match ["th".into(), "sh".into(), "ch".into()]
        .contains(&input.chars().take(2).collect::<String>()) {
        false => input.chars().take(1).collect::<String>(),
        true => input.chars().take(2).collect::<String>()

    }
}

fn skip_first(input: &str) -> String {
    match ["th".into(), "sh".into(), "ch".into()]
        .contains(&input.chars().take(2).collect::<String>()) {
        false => input.chars().skip(1).collect::<String>(),
        true => input.chars().skip(2).collect::<String>()

    }
}

fn has_qu(input: &str) -> bool {
    &skip_first(input).chars().skip(1).take(2).collect::<String>() == "qu"
}