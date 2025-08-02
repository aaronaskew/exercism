#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // sanitize input

    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    for digit in number {
        if *digit > from_base - 1 {
            return Err(Error::InvalidDigit(*digit));
        }
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    // convert

    let mut digits = number.to_vec();
    digits.reverse();

    let mut value = 0;

    for (i, digit) in digits.iter().enumerate() {
        value += digit * from_base.pow(i as u32);
    }

    let mut converted_digits = Vec::new();

    let mut position = 0;

    loop {
        position += 1;

        if to_base.pow(position) > value {
            position -= 1;
            break;
        }
    }

    dbg!(&to_base, &position, &value, to_base.pow(position));

    loop {
        let digit = value / to_base.pow(position);
        dbg!(&digit);

        value -= digit * to_base.pow(position);
        dbg!(&value);

        // dbg!(&digit, &value, &position);

        converted_digits.push(digit);

        dbg!(&converted_digits);

        if position == 0 {
            break;
        }

        position = position.saturating_sub(1);

        dbg!(&position);
    }

    Ok(converted_digits)
}
