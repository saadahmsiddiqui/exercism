use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

/// Convert a word into a sorted list of unicode graphemes

fn convert_to_anagram(word: &str) -> Vec<String> {
    let mut result: Vec<String> = word.graphemes(true).map(|c| c.to_uppercase()).collect();

    result.sort();

    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_anagram = convert_to_anagram(word);

    possible_anagrams
        .iter()
        .cloned() // converts &a to a
        .filter(|anagram| {
            convert_to_anagram(anagram) == word_anagram
                && word.to_uppercase() != anagram.to_uppercase()
        })
        .collect()
}
