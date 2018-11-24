

const VOWELS: [char; 5] = ['a', 'e', 'u', 'i', 'o'];

const TRIPLE_CONSONANTS: [&str; 2] = ["thr", "sch"];

const DOUBLE_CONSONANTS: [&str; 3] = ["th", "sh", "ch"];


pub fn translate(input: &str) -> String {
    let mut words = vec![];

    input.split(" ").for_each(|word| {
        words.push(translate_word(word));
    });

    words.join(" ")
}

fn translate_word(input: &str) -> String {
    match first_is_vowel(&input) {
        true => return vowel_ay(&input),
        false => {
            match has_consonant_qu(&input) {
                true => {
                    println!("hasqu alright!!");
                    return consonant_qu_swap(&input)
                },
                false => {
                    match has_qu(input) {
                        true => {
                            println!("simple qu");
                            return qu_swap(input)
                        },
                        false => {
                            println!("time for consonant swap");
                            return consonant_swap(&input)
                        }
                    };

                }
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

fn qu_swap(input: &str) -> String {
    format!("{}quay", input.chars().skip(2).collect::<String>())

}

fn first_is_vowel(input: &str) -> bool {
    input.chars().take(1).any(|x| VOWELS.contains(&x))
        || ["yt".into(), "xr".into()]
            .contains(&input.chars().take(2).collect::<String>())
}

fn get_first_unsafe(input: &str) -> String {
    match TRIPLE_CONSONANTS
        .contains(&input.chars().take(3).collect::<String>().as_str()) {
            false => {
                match DOUBLE_CONSONANTS
                .contains(&input.chars().take(2).collect::<String>().as_str()) {
                false => input.chars().take(1).collect::<String>(),
                true => input.chars().take(2).collect::<String>()
                }
            },
            true => input.chars().take(3).collect::<String>()
    }
}

fn skip_first(input: &str) -> String {
    match TRIPLE_CONSONANTS
        .contains(&input.chars().take(3).collect::<String>().as_str()) {
        false =>    {
            match DOUBLE_CONSONANTS
                .contains(&input.chars().take(2).collect::<String>().as_str()) {
                false => input.chars().skip(1).collect::<String>(),
                true => input.chars().skip(2).collect::<String>()
                }
            }
        true => input.chars().skip(3).collect::<String>()
    }
}


fn has_consonant_qu(input: &str) -> bool {
    &skip_first(input).chars().take(2).collect::<String>() == "qu"
}

fn has_qu(input: &str) -> bool {
    &input.chars().take(2).collect::<String>() == "qu"
}