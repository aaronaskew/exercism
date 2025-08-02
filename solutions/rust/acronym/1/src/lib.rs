pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    phrase
        .split([' ', '-'])
        .map(|x| x.to_string())
        .map(|x| x.replace(|c: char| !c.is_ascii_alphanumeric(), ""))
        .filter(|x| !x.is_empty())
        .for_each(|word| {
            word.chars().enumerate().for_each(|(i, c)| {
                if i == 0 {
                    result.push(c.to_ascii_uppercase());
                } else if c.is_ascii_uppercase()
                    && word.chars().nth(i - 1).unwrap().is_ascii_lowercase()
                {
                    result.push(c);
                }
            })
        });

    dbg!(result)
}
