use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::<u32>::new();

    factors.iter().for_each(|f| {
        let mut multiplier = 1;

        while f * multiplier < limit && *f != 0 {
            multiples.insert(f * multiplier);
            multiplier += 1;
        }
    });

    dbg!(&multiples);

    dbg!(multiples.iter().sum::<u32>())

    // todo!()
}
