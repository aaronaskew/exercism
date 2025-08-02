pub fn series(digits: &str, len: usize) -> Vec<String> {
    let vec: Vec<char> = digits.chars().collect();

    vec.windows(len).map(|chs| chs.iter().collect()).collect()
}
