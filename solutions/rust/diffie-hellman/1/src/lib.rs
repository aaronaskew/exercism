use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result = 1_u128;
    let mut base = b as u128 % m as u128;
    let mut exponent = e as u128;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % m as u128;
        }
        exponent = exponent >> 1;
        base = (base * base) % m as u128;
    }

    result.try_into().unwrap()
}
