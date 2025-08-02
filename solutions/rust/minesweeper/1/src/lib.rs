use itertools::Itertools;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    dbg!(minefield);

    let rows = minefield.len();

    if rows > 0 {
        let cols = minefield[0].len();

        dbg!(rows, cols);

        minefield
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(col, char)| {
                        let mut search_positions: Vec<(usize, usize)> = Vec::new();

                        match char {
                            b'*' => '*'.to_string(),
                            _ => {
                                if row > 0 && col > 0 {
                                    search_positions.push((row - 1, col - 1));
                                }
                                if row > 0 {
                                    search_positions.push((row - 1, col));
                                }
                                if row > 0 && col < cols - 1 {
                                    search_positions.push((row - 1, col + 1));
                                }
                                if col > 0 {
                                    search_positions.push((row, col - 1));
                                }
                                if col < cols - 1 {
                                    search_positions.push((row, col + 1));
                                }
                                if row < rows - 1 && col > 0 {
                                    search_positions.push((row + 1, col - 1));
                                }
                                if row < rows - 1 {
                                    search_positions.push((row + 1, col));
                                }
                                if row < rows - 1 && col < cols - 1 {
                                    search_positions.push((row + 1, col + 1));
                                }

                                let count = search_positions.iter().fold(0, |acc, (row, col)| {
                                    acc + match minefield[*row].as_bytes()[*col] {
                                        b'*' => 1,
                                        _ => 0,
                                    }
                                });

                                if count > 0 {
                                    count.to_string()
                                } else {
                                    ' '.to_string()
                                }
                            }
                        }
                    })
                    .join("")
            })
            .collect()
    } else {
        vec![]
    }
}
