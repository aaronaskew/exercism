use std::collections::HashSet;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        (is_palindrome(value)).then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn is_palindrome(value: u64) -> bool {
    value.to_string() == value.to_string().chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut palindrome_min = None;
    let mut palindrome_max = None;

    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }

        for j in min..=max {
            if j % 10 == 0 {
                continue;
            }

            let product = i * j;

            if is_palindrome(product) {
                if product < palindrome_min.unwrap_or(u64::MAX) {
                    palindrome_min = Some(product);
                }
                if product > palindrome_max.unwrap_or(u64::MIN) {
                    palindrome_max = Some(product);
                }
            }
        }
    }

    match (palindrome_min, palindrome_max) {
        (None, None) => None,
        (None, Some(large)) => Some((
            Palindrome::new(large).unwrap(),
            Palindrome::new(large).unwrap(),
        )),
        (Some(small), None) => Some((
            Palindrome::new(small).unwrap(),
            Palindrome::new(small).unwrap(),
        )),
        (Some(small), Some(large)) => Some((
            Palindrome::new(small).unwrap(),
            Palindrome::new(large).unwrap(),
        )),
    }
}
