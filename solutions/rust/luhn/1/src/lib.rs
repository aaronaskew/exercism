/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let no_whitespace: String = code.split_whitespace().collect();

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
                if num > 9 {
                    num - 9
                } else {
                    num
                }
            } else {
                num
            }
        })
        .inspect(|x| {
            dbg!(x);
        })
        .sum::<u32>();

    dbg!(sum);

    sum % 10 == 0
}
