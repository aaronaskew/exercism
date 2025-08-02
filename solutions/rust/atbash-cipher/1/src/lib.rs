fn atbash(c: char) -> char {
    let i = c.to_ascii_lowercase() as u8 - b'a';
    let j = 26 - i - 1;
    (j + b'a') as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            c if c.is_numeric() => c,
            _ => atbash(c),
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chars| chars.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| match c {
            c if c.is_numeric() => c,
            _ => atbash(c),
        })
        .collect()
}
