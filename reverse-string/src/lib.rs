use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();

    // String::from_utf8(bts_vec).unwrap()
    String::from("")
}
