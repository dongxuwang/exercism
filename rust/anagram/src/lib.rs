use std::collections::HashSet;
use std::iter::FromIterator;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sort(&word_lower);

    possible_anagrams.iter().filter(|x| {
        let x_lower = x.to_lowercase();
        let x_sorted = sort(&x_lower);

        x_lower.len() == word_lower.len() && x_lower != word_lower && x_sorted == word_sorted

    }).copied().collect()
}

fn sort(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
