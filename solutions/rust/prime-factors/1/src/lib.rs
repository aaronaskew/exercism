pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::<u64>::new();

    let mut divisor = 2u64;

    let mut dividend = n;

    while dividend > 1 {
        if dividend % divisor == 0 {
            factors.push(divisor);
            dividend /= divisor;
        } else {
            divisor += 1;
        }
    }

    factors
}
