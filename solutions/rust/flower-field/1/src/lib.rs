pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    if garden[0].is_empty() {
        return vec![String::new()];
    }

    let rows = garden.len();
    let cols = garden[0].len();

    let mut garden = garden
        .iter()
        .map(|&row| {
            let row = row.as_bytes();
            row.to_vec()
        })
        .collect::<Vec<Vec<_>>>();

    for j in 0..rows {
        for i in 0..cols {
            if garden[j][i] == b'*' {
                continue;
            }

            let flowers = count_flowers(&garden, j, i, rows, cols);
            if flowers == 0 {
                continue;
            } else {
                garden[j][i] = b'0' + flowers;
            }
        }
    }

    garden
        .into_iter()
        .map(|row| String::from_utf8_lossy(&row).to_string())
        .collect()
}

fn count_flowers(garden: &[Vec<u8>], row: usize, col: usize, rows: usize, cols: usize) -> u8 {
    const DIRECTIONS: [(i8, i8); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let flower = b'*';

    DIRECTIONS
        .iter()
        .map(|direction| {
            let new_row = row as i32 + direction.0 as i32;
            let new_col = col as i32 + direction.1 as i32;

            if (0..rows).contains(&(new_row as usize)) && (0..cols).contains(&(new_col as usize)) {
                if garden[new_row as usize][new_col as usize] == flower {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}
