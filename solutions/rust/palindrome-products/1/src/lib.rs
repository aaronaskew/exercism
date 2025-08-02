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
        (value.to_string() == value.to_string().chars().rev().collect::<String>())
            .then_some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut palindromes: HashSet<Palindrome> = HashSet::new();

    for i in min..=max {
        for j in min..=max {
            if let Some(product) = Palindrome::new(i * j) {
                palindromes.insert(product);
            }
        }
    }

    match (palindromes.iter().min(), palindromes.iter().max()) {
        (Some(&s), Some(&l)) => Some((s, l)),
        _ => None,
    }
}
