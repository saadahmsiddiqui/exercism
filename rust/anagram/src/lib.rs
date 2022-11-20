use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    for anagram in possible_anagrams {
        let word_chars: Vec<char> = word.chars().map(|x| x.to_ascii_lowercase()).collect();
        let anagram_chars: Vec<char> = anagram.chars().map(|x| x.to_ascii_lowercase()).collect();

        if word_chars.len() != anagram_chars.len() {
            continue;
        }

        let mut all_match = true;
        for ch in anagram_chars.iter() {
            if !word_chars.contains(&ch) {
                all_match = false;
            }
        }

        if all_match {
            anagrams.insert(anagram.clone());
        }
    }

    return anagrams;
}
