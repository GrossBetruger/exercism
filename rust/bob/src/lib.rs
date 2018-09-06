
extern crate foreach;

use foreach::*;

pub fn reply(message: &str) -> &str {
    let message = message.trim_right();
    let suffix: String = message.chars().rev().take(1).into_iter().collect();
    let all_caps_message = all_caps(&message);

    println!("message '{}'", message);
    println!("suffix '{}'", suffix);
    println!("ALLCAPS {}", all_caps_message);

    match all_caps_message {
        false => {
              match suffix.as_str() {
//                "." => "Whatever.",
                "!" => "Whoa, chill out!",
                "?" => "Sure.",
                "" => "Fine. Be that way!",
                _ => "Whatever.",
            }
        },
        true => match suffix.as_str() {
            "?" => "Calm down, I know what I'm doing!",
            _ => "Whoa, chill out!",
        }
    }
}

fn all_caps(message: &str) -> bool {
    let mut all_caps = true;
    let chr_it = message.chars();
    let alpha = chr_it.filter(|chr| chr.is_alphabetic()).collect::<String>();
    if alpha.len() == 0 {
        return false;
    }
    alpha.chars()
        .filter(move |chr | chr.is_alphabetic())
        .foreach(|chr, _| {
            if  chr.is_lowercase() {
                all_caps = false;
            }
    });
    all_caps
    }


