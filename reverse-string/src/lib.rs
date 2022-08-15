use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let g = input.graphemes(true).collect::<Vec<&str>>();

    // String::from_utf8(bts_vec).unwrap()
    String::from("")
}
