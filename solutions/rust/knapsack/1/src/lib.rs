#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let n = items.len();

    let mut dp = vec![vec![0; max_weight as usize + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=max_weight as usize {
            let item = &items[i - 1];

            if item.weight <= w as u32 {
                dp[i][w] = std::cmp::max(
                    dp[i - 1][w],
                    dp[i - 1][w - item.weight as usize] + item.value,
                );
            } else {
                dp[i][w] = dp[i - 1][w];
            }
        }
    }

    dp[n][max_weight as usize]
}
