pub fn raindrops(n: usize) -> String {
   let mut rainspeak = String::from("");

    if n % 3 == 0 {
        rainspeak.push_str("Pling");
    }

    if n % 5 == 0 {
        rainspeak.push_str("Plang");

    }

    if n % 7 == 0 {
        rainspeak.push_str("Plong");
    }

    if rainspeak.is_empty() {
        return n.to_string();
    }

    rainspeak
}
