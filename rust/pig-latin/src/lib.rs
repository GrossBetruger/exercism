

const VOWELS: [char; 5] = ['a', 'e', 'u', 'i', 'o'];

pub fn translate(input: &str) -> String {
   let pig_latin = vowel_ay(input);
    pig_latin
}

fn vowel_ay(input: &str) -> String {
    match input.chars().take(1).any(|x| VOWELS.contains(&x)) {
        true => format!("{}{}", input, "ay"),
        false => input.into()
    }
}