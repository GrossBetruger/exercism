
extern crate foreach;

use foreach::*;

pub fn reply(message: &str) -> &str {
    let suffix: String = message.chars().rev().take(1).into_iter().collect();
    let mut all_caps = false;

    let chr_it = message.chars();
    if chr_it.filter(|chr | chr.is_alphabetic()).collect::<String>().len() > 0 {

        chr_it
        .filter(move |chr | chr.is_alphabetic())
        .foreach(|chr, _| {
            if  chr.is_uppercase() {
                all_caps = true;
            }
    });


    }

    if !vec!["?", "!", "."].contains(&suffix.as_str()) {
        panic!("bad grammar, message must end with '?' or '!'");
    }

    println!("message {}", message);
    println!("suffix '{}'", suffix);
    println!("ALLCAPS {}", all_caps);

    match all_caps {
        false => {
              match suffix.as_str() {
                "." => "Whatever.",
                "!" => "Whoa, chill out!",
                "?" => "Sure.",
                "" => "Fine. Be that way!",
                _ => panic!("bad grammar, message must end with '?' or '!' or '.'")
            }
        },
        true => match suffix.as_str() {
            "." => "Whatever.",
            "!" => "Whoa, chill out!",
            "?" => "Calm down, I know what I'm doing!",
            "" => "Fine. Be that way!",
            _ => panic!("bad grammar, message must end with '?' or '!' or '.'")

        }
    }
}