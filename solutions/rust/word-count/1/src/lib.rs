use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut chars = words.chars().collect::<Vec<_>>();

    // strip bad apostrophes
    for i in 0..chars.len() {
        if chars[i] == '\''
            && (i == 0
                || i == chars.len() - 1
                || !(chars[i - 1].is_alphanumeric() && chars[i + 1].is_alphanumeric()))
        {
            chars[i] = ' ';
        }
    }

    // replace punctuation with spaces as make lowercase
    for c in chars.iter_mut() {
        if c.is_alphanumeric() || *c == '\'' {
            *c = c.to_ascii_lowercase();
        } else {
            *c = ' ';
        }
    }

    let mut count = HashMap::new();

    chars
        .iter()
        .collect::<String>()
        .split_whitespace()
        .for_each(|word| {
            count
                .entry(word.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    count
}
