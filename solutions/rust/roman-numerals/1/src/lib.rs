use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(
            [
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (90, "XC"),
                (50, "L"),
                (40, "XL"),
                (10, "X"),
                (9, "IX"),
                (5, "V"),
                (4, "IV"),
                (1, "I"),
            ]
            .into_iter()
            .fold((num, String::new()), |(num, roman), (value, numeral)| {
                let mut num = num;
                let mut roman = roman;

                while num >= value {
                    num -= value;
                    roman += numeral;
                }

                (num, roman)
            })
            .1,
        )
    }
}
