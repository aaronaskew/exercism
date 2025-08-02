use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut letters: HashMap<char, u64> = HashMap::new();
    letters.insert('A', 1);
    letters.insert('E', 1);
    letters.insert('I', 1);
    letters.insert('O', 1);
    letters.insert('U', 1);
    letters.insert('L', 1);
    letters.insert('N', 1);
    letters.insert('R', 1);
    letters.insert('S', 1);
    letters.insert('T', 1);
    letters.insert('D', 2);
    letters.insert('G', 2);
    letters.insert('B', 3);
    letters.insert('C', 3);
    letters.insert('M', 3);
    letters.insert('P', 3);
    letters.insert('F', 4);
    letters.insert('H', 4);
    letters.insert('V', 4);
    letters.insert('W', 4);
    letters.insert('Y', 4);
    letters.insert('K', 5);
    letters.insert('J', 8);
    letters.insert('X', 8);
    letters.insert('Q', 10);
    letters.insert('Z', 10);

    word.chars().fold(0, |acc, c| {
        acc + letters.get(&c.to_ascii_uppercase()).unwrap_or(&0)
    })
}
