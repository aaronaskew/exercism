use rand::Rng;

fn shift(c: char) -> u8 {
    c as u8 - b'a'
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| c.is_uppercase() || !c.is_alphabetic()) {
        None
    } else {
        let key = key.chars().collect::<Vec<_>>();

        Some(
            s.chars()
                .enumerate()
                .map(|(i, c)| ((c as u8 + shift(key[i % key.len()]) - b'a') % 26 + b'a') as char)
                .collect(),
        )
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| c.is_uppercase() || !c.is_alphabetic()) {
        None
    } else {
        let key = key.chars().collect::<Vec<_>>();

        Some(
            s.chars()
                .enumerate()
                .map(|(i, c)| {
                    ((c as u8 + 26 - shift(key[i % key.len()]) - b'a') % 26 + b'a') as char
                })
                .collect(),
        )
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();
    let mut key = String::new();

    for _ in 0..100 {
        key.push(rng.random_range('a'..='z'));
    }

    (key.clone(), encode(key.as_str(), s).unwrap())
}
