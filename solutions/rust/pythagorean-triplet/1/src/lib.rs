use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 0..sum {
        for b in (a + 1)..sum {
            if sum >= a + b {
                let c = sum - a - b;
                if c > b && a * a + b * b == c * c {
                    triplets.insert([a, b, c]);
                }
            }
        }
    }

    triplets
}
