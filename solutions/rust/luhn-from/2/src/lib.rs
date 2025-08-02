pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let no_whitespace: String = self.code.split_whitespace().collect();

        dbg!(&no_whitespace);

        if no_whitespace.len() < 2 {
            return false;
        }

        for c in no_whitespace.chars() {
            if !c.is_ascii_digit() {
                return false;
            }
        }

        let sum = no_whitespace
            .chars()
            .rev()
            .enumerate()
            .map(|(i, ch)| {
                let num = ch.to_digit(10).unwrap();

                if i % 2 == 1 {
                    let num = num * 2;
                    if num > 9 { num - 9 } else { num }
                } else {
                    num
                }
            })
            .inspect(|x| {
                dbg!(x);
            })
            .sum::<u32>();

        dbg!(sum);

        sum.is_multiple_of(10)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
