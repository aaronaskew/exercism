use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let numbers = HashMap::from([
        (
            vec![
                vec![' ', '_', ' '],
                vec!['|', ' ', '|'],
                vec!['|', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            0u8,
        ),
        (
            vec![
                vec![' ', ' ', ' '],
                vec![' ', ' ', '|'],
                vec![' ', ' ', '|'],
                vec![' ', ' ', ' '],
            ],
            1,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec![' ', '_', '|'],
                vec!['|', '_', ' '],
                vec![' ', ' ', ' '],
            ],
            2,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec![' ', '_', '|'],
                vec![' ', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            3,
        ),
        (
            vec![
                vec![' ', ' ', ' '],
                vec!['|', '_', '|'],
                vec![' ', ' ', '|'],
                vec![' ', ' ', ' '],
            ],
            4,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec!['|', '_', ' '],
                vec![' ', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            5,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec!['|', '_', ' '],
                vec!['|', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            6,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec![' ', ' ', '|'],
                vec![' ', ' ', '|'],
                vec![' ', ' ', ' '],
            ],
            7,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec!['|', '_', '|'],
                vec!['|', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            8,
        ),
        (
            vec![
                vec![' ', '_', ' '],
                vec!['|', '_', '|'],
                vec![' ', '_', '|'],
                vec![' ', ' ', ' '],
            ],
            9,
        ),
    ]);

    let grid: Vec<Vec<char>> = input
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    if !rows.is_multiple_of(4) {
        return Err(Error::InvalidRowCount(rows));
    }

    for row in &grid {
        let num_cols = row.len();
        if !row.len().is_multiple_of(3) {
            return Err(Error::InvalidColumnCount(cols));
        }
        assert!(num_cols == cols);
    }

    let mut result = String::new();

    for j in 0..(rows / 4) {
        for i in 0..(cols / 3) {
            let digit = vec![
                grid[j * 4][i * 3..(i * 3 + 3)].to_vec(),
                grid[j * 4 + 1][i * 3..(i * 3 + 3)].to_vec(),
                grid[j * 4 + 2][i * 3..(i * 3 + 3)].to_vec(),
                grid[j * 4 + 3][i * 3..(i * 3 + 3)].to_vec(),
            ];

            if let Some(value) = numbers.get(&digit) {
                result.push_str(&value.to_string());
            } else {
                result.push('?');
            }
        }

        if j < (rows / 4 - 1) {
            result.push(',');
        }
    }

    Ok(result)
}
