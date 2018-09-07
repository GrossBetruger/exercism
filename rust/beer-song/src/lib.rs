const VERSE_HEAD: &str = "{} bottles of beer on the wall, {} bottles of beer.\n";

const VERSE_REGULAR_TAIL: &str = "Take one down and pass it around, {} bottles of beer on the wall.\n";

const VERSE_ENDING_TAIL: &str = "Go to the store and buy some more, 99 bottles of beer on the wall.\n";

pub fn verse(n: i32) -> String {
    let formatted_n = format_num(n);
    let formatted_n_minus_one = format_num(n -1);
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.".into(),
        1 => format!("{} of beer on the wall, {} of beer.
Take it down and pass it around, {} of beer on the wall.", formatted_n, formatted_n, formatted_n_minus_one),
        _ => format!("{} of beer on the wall, {} of beer.
Take one down and pass it around, {} of beer on the wall.", formatted_n, formatted_n, formatted_n_minus_one)
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    for i in (end..start+1_i32).rev() {
        song.push_str(&verse(i));
        match i == end {
            true  =>  song.push('\n'),
            false =>  song.push_str("\n\n"),
        }
//        song.push_str("\n\n");
    }
    println!("SONG:\n{}", verse(1));
//    song.push_str("\n");
    song
}

fn format_num(n: i32) -> String {
    match n {
        0 => "no more bottles".into(),
        1 => format!("{} {}", n.to_string(), "bottle"),
        _ => format!("{} {}", n.to_string(), "bottles")

    }
}
