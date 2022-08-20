use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    g.reverse();

    let string_vc = g.iter().map(|ch| String::from(*ch)).collect::<Vec<String>>();
    let mut final_string = String::from("");

    for ch in string_vc.iter() {
        final_string = final_string + ch;
    }

    final_string
}
