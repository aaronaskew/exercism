#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        Err(Error::SpanTooLong)
    } else {
        let mut digits: Vec<u64> = Vec::new();

        for c in string_digits.chars() {
            if let Ok(digit) = c.to_string().parse() {
                digits.push(digit)
            } else {
                return Err(Error::InvalidDigit(c));
            }
        }

        Ok(digits
            .windows(span)
            .map(|window| window.iter().product())
            .max()
            .unwrap())
    }
}
