pub fn egg_count(display_value: u32) -> usize {
    let mut num_bits = 0;

    let mut decimal = display_value;

    for i in (0..32).rev() {
        let power_of_2 = 2u32.pow(i);

        if power_of_2 <= decimal {
            num_bits += 1;
            decimal -= power_of_2;
        }
    }

    num_bits
}
