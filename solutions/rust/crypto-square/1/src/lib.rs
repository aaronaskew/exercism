fn rows_columns(input: &str) -> (usize, usize) {
    let mut r = 1;
    let mut c = 1;

    loop {
        if r * c >= input.len() && c >= r && c - r <= 1 {
            break;
        }

        if c == r {
            c += 1;
        } else {
            r += 1;
        }
    }

    (r, c)
}

fn normalize(input: &str) -> String {
    input
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect()
}

pub fn encrypt(input: &str) -> String {
    let mut cipher = String::new();

    let mut normalized = normalize(input).chars().rev().collect::<String>();

    if normalized.is_empty() {
        return cipher;
    }

    let (r, c) = rows_columns(&normalized);

    dbg!(&r, &c);

    let mut rectangle = vec![vec![' '; c]; r];

    for j in 0..r {
        for i in 0..c {
            if let Some(ch) = normalized.pop() {
                rectangle[j][i] = ch;
            } else {
                rectangle[j][i] = ' ';
            }
        }
    }

    dbg!(&rectangle);

    for i in 0..c {
        for j in 0..r {
            cipher.push(rectangle[j][i]);
            if j == r - 1 && i != c - 1{
                cipher.push(' ');
            }
        }
    }

    cipher
}
