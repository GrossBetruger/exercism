extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let res: String = input
        .graphemes(true)
        .rev()
        .flat_map(|graph| graph.chars())
        .collect();
    res
}
