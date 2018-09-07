const VERSE_HEAD: &str = "{} bottles of beer on the wall, {} bottles of beer.\n";

const VERSE_REGULAR_TAIL: &str = "Take one down and pass it around, {} bottles of beer on the wall.\n";

const VERSE_ENDING_TAIL: &str = "Go to the store and buy some more, 99 bottles of beer on the wall.\n";

pub fn verse(n: i32) -> String {
    match n {
        0 => format!("{}{}",
                     format!("{} of beer on the wall, {} of beer.\n",
                             format_num(n), format_num(n)),
                     "Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => format!("{}{}", format!("{} of beer on the wall, {} of beer.\n",
                                     format_num(n), format_num(n)),
                     format!("Take one down and pass it around, {} of beer on the wall.\n",
                             format_num(n - 1)))
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for i in (end..start+1_i32).rev() {
        song.push_str(&verse(i))
    }
    println!("SONG:\n{}", verse(1));
    song
}

fn format_num(n: i32) -> String {
    match n {
        0 => "no more bottles".into(),
        1 => format!("{} {}", n.to_string(), "bottle"),
        _ => format!("{} {}", n.to_string(), "bottles")

    }
}
