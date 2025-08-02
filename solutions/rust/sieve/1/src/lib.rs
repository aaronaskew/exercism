pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }

    let mut numbers: Vec<u64> = (2..=upper_bound).collect();

    for number in numbers.clone() {
        let mut i = number;
        loop {
            if i > upper_bound {
                break;
            }

            let product = number * i;
            if product <= upper_bound {
                if let Ok(idx) = numbers.binary_search(&product) {
                    numbers.remove(idx);
                }
            }
            i += 1;
        }
    }

    numbers
}
