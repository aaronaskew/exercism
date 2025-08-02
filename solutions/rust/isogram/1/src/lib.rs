use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters: HashSet<char> = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(
            true,
            |is_iso, c| {
                if letters.insert(c) {
                    is_iso
                } else {
                    false
                }
            },
        )
}
