pub fn is_armstrong_number(num: u32) -> bool {
    
    let num_str = num.to_string();
    let num_len = num_str.len();

    let mut sum = 0;
    for c in num_str.chars() {
        let num = c.to_digit(10).unwrap() as u64;
        sum += num.pow(num_len as u32);
    }
    
    num as u64== sum

}
