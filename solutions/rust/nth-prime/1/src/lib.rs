pub fn nth(n: u32) -> u32 {
   

    let mut primes = Vec::<u32>::new();

    let mut num = 2;
    while primes.len() < n as usize + 1 {
        if is_prime(num) {
            primes.push(num);
        }
        num += 1;
    }

    dbg!(&primes);

    *primes.last().unwrap()
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..(n as f64).sqrt() as u32 + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
