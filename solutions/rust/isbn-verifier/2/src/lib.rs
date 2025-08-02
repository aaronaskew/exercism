pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .try_fold((0_u32, 10_u32), |(sum, weight), c| {
            (c == 'X' && weight == 1)
                .then_some(10)
                .or_else(|| c.to_digit(10))
                .and_then(|number| {
                    weight
                        .checked_sub(1)
                        .map(|new_weight| (sum + number * weight, new_weight))
                })
        })
        .map_or(false, |(sum, weight)| sum % 11 == 0 && weight == 0)
}
