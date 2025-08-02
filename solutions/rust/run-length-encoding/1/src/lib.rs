pub fn encode(source: &str) -> String {
    let mut encoded = String::new();

    if source.is_empty() {
        return encoded;
    }

    let mut curr_char = None;
    let mut curr_count = 0;

    for c in source.chars() {
        if curr_char.is_none() {
            curr_char = Some(c);
            curr_count = 1;
        } else if c != curr_char.unwrap() {
            if curr_count == 1 {
                encoded.push(curr_char.unwrap());
            } else {
                encoded.push_str(&format!("{curr_count}{}", curr_char.unwrap()))
            }
            curr_char = Some(c);
            curr_count = 1;
        } else {
            curr_count += 1;
        }
    }

    if curr_count == 1 {
        encoded.push(curr_char.unwrap());
    } else {
        encoded.push_str(&format!("{curr_count}{}", curr_char.unwrap()))
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count_string = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            count_string.push(c);
        } else if let Ok(count) = count_string.parse::<i32>() {
            for _ in 0..count {
                decoded.push(c);
            }
            count_string.clear();
        } else {
            decoded.push(c);
        }
    }

    decoded
}
