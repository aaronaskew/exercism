/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut isbn_chars = isbn.chars().collect::<Vec<char>>();
    isbn_chars.retain(|c| c.is_ascii_alphanumeric());

    // check length
    if isbn_chars.len() != 10 {
        return false;
    }

    // check for valid characters
    let mut valid_chars = true;
    isbn_chars.iter().enumerate().for_each(|(i, c)| {
        if i < 9 && !c.is_ascii_digit() {
            println!("i<9 & !digit");
            valid_chars = false;
        }

        if i == 9 && !c.is_ascii_digit() && *c != 'X' {
            println!("i=9 & !digit or !X");
            valid_chars = false;
        }
    });
    if !valid_chars {
        return false;
    }

    // convert chars to numbers
    let numbers: Vec<u32> = isbn_chars
        .iter()
        .map(|c| {
            if *c == 'X' {
                10
            } else {
                c.to_digit(10).unwrap()
            }
        })
        .collect();

    // check for ISBN validity
    (numbers[0] * 10
        + numbers[1] * 9
        + numbers[2] * 8
        + numbers[3] * 7
        + numbers[4] * 6
        + numbers[5] * 5
        + numbers[6] * 4
        + numbers[7] * 3
        + numbers[8] * 2
        + numbers[9])
        % 11
        == 0
}
