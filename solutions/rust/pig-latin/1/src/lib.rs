#![feature(iter_intersperse)]

pub fn translate(input: &str) -> String {
    let words = input.split_whitespace();

    words
        .map(pig_latinify_word)
        .intersperse(" ".to_string())
        .collect()
}

fn starting_consonants(input: &str) -> usize {
    if input.is_empty()
        || input.starts_with('a')
        || input.starts_with('e')
        || input.starts_with('i')
        || input.starts_with('o')
        || input.starts_with('u')
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        0
    } else if input.starts_with("qu") {
        2
    } else if input.chars().nth(1) == Some('y') {
        1
    } else {
        1 + starting_consonants(&input[1..])
    }
}

fn pig_latinify_word(word: &str) -> String {
    let (consonants, remainder) = word.split_at(starting_consonants(word));
    dbg!(consonants, remainder);
    format!("{}{}ay", remainder, consonants)
}
