#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (gcd, x, y)
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        None
    } else {
        Some(((x % m) + m) % m)
    }
}

fn is_coprime(a: i32, b: i32) -> bool {
    matches!(extended_gcd(a, b), (1, _, _))
}

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn c_to_i(ch: char) -> Option<i32> {
    ALPHABET.iter().position(|c| *c == ch).map(|i| i as i32)
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut cipher = String::new();

    let mut space_count = 0;
    for ch in plaintext.chars() {
        let add_space = !cipher.is_empty() && (cipher.len() - space_count).is_multiple_of(5);

        let plain = ch.to_ascii_lowercase();
        if plain.is_alphanumeric() {
            if add_space {
                cipher.push(' ');
                space_count += 1;
            }

            if let Some(i) = c_to_i(plain) {
                cipher.push(ALPHABET[((a * i + b) % 26) as usize]);
            } else if plain.is_ascii_digit() {
                cipher.push(plain);
            }
        }
    }

    Ok(cipher)
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let mut plain = String::new();
    if let Some(x) = mod_inverse(a, 26) {
        for ch in ciphertext.chars() {
            if ch.is_alphabetic()
                && let Some(mut y) = c_to_i(ch)
            {
                while y - b < 0 {
                    y += 26;
                }
                plain.push(ALPHABET[x as usize * (y - b + 26) as usize % 26])
            } else if ch.is_numeric() {
                plain.push(ch);
            }
        }
    } else {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plain)
}
