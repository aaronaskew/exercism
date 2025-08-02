const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn encode(n: u64) -> String {
    let mut number = n;
    let mut thousands_list = Vec::new();

    while number > 0 {
        thousands_list.push(number % 1000);
        number /= 1000;
    }

    if thousands_list.len() <= 1 {
        match n {
            0..=9 => ONES[n as usize].to_string(),
            10..=19 => TEENS[n as usize - 10].to_string(),
            20..=99 if n % 10 == 0 => TENS[n as usize / 10].to_string(),
            20..=99 => format!("{}-{}", TENS[n as usize / 10], ONES[n as usize % 10]),
            100..=999 if n % 100 == 0 => format!("{} hundred", ONES[n as usize / 100]),
            100..=999 => format!("{} hundred {}", ONES[n as usize / 100], encode(n % 100)),
            _ => todo!(),
        }
    } else {
        thousands_list.reverse();

        let mut result = String::new();

        for (i, &number) in thousands_list.iter().enumerate() {
            let magnitude = match thousands_list.len() - i {
                0 | 1 => "",
                2 => "thousand",
                3 => "million",
                4 => "billion",
                5 => "trillion",
                6 => "quadrillion",
                7 => "quintillion",
                _ => panic!("Invalid magnitude"),
            };
            match number {
                0 => (),
                _ => {
                    let number_str = encode(number);
                    result.push_str(&format!("{} {} ", number_str, magnitude));
                }
            }
        }

        result.trim().to_string()
    }
}
