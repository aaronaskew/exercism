use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_sorted = word.to_lowercase().chars().collect::<Vec<char>>();
    word_sorted.sort_unstable();

    dbg!(&word_sorted);

    possible_anagrams
        .iter()
        .map(|possible_anagram| {
            let mut sorted = possible_anagram
                .to_lowercase()
                .chars()
                .collect::<Vec<char>>();
            sorted.sort_unstable();

            AnagramCandidate {
                original: possible_anagram,
                sorted,
            }
        })
        .filter(|candidate| {
            candidate.sorted == word_sorted
                && candidate.original.to_lowercase() != word.to_lowercase()
        })
        .inspect(|candidate| {
            dbg!(candidate);
        })
        .map(|candidate| candidate.original)
        .collect()

    //todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
}

#[derive(Debug, Clone)]
struct AnagramCandidate<'a> {
    original: &'a str,
    sorted: Vec<char>,
}
