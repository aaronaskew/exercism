pub fn number(user_number: &str) -> Option<String> {
    let mut digits = user_number
        .chars()
        .filter(|ch: &char| ch.is_ascii_digit())
        .collect::<Vec<_>>();

    if digits.len() == 11 {
        if digits[0] != '1' {
            return None;
        } else {
            digits = Vec::from(&digits[1..]);
        }
    }

    if digits.len() != 10 {
        return None;
    }

    if matches!(digits[0], '0' | '1') || matches!(digits[3], '0' | '1') {
        return None;
    }

    Some(digits.iter().collect())
}
