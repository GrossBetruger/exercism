pub fn encode(source: &str) -> String {
    let mut characters = source.chars();
    let mut last_char = characters.next().unwrap();//characters.by_ref().take(1).collect::<Vec<char>>().get(0).unwrap().to_owned();
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
                    _ => compressed.push_str(&format!("{}{}", count, last_char))
                }
                last_char = c;
                count = 1;
            }

        }
    }

    compressed

}

pub fn decode(source: &str) -> String {
    "not yet impl".into()
}


