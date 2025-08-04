pub fn count(lines: &[&str]) -> u32 {
    let grid = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let rows = grid.len();
    if rows == 0 {
        return 0;
    }

    let cols = grid[0].len();
    if cols == 0 {
        return 0;
    }

    let mut count = 0;

    for j in 0..rows {
        for i in 0..cols {
            if grid[j][i] == '+' {
                let top_left = (j, i);

                if top_left.0 + 1 < rows {
                    for ii in top_left.1 + 1..cols {
                        if grid[j][ii] == '+' {
                            let top_right = (j, ii);

                            if top_left.0 + 1 < rows {
                                for jj in top_left.0 + 1..rows {
                                    if grid[jj][i] == '+' {
                                        let bot_left = (jj, i);

                                        if grid[jj][ii] == '+' {
                                            let bot_right = (jj, ii);

                                            if is_complete_rectangle(
                                                &grid, &top_left, &top_right, &bot_left, &bot_right,
                                            ) {
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn is_complete_rectangle(
    grid: &[Vec<char>],
    top_left: &(usize, usize),
    top_right: &(usize, usize),
    bot_left: &(usize, usize),
    bot_right: &(usize, usize),
) -> bool {
    // left side
    grid.iter()
        .skip(top_left.0)
        .take(bot_left.0 - top_left.0 + 1)
        .all(|v| {
            v.iter()
                .skip(top_left.1)
                .take(1)
                .all(|ch| *ch == '+' || *ch == '|')
        })
        // right side
        && grid
            .iter()
            .skip(top_right.0)
            .take(bot_right.0 - top_right.0 + 1)
            .all(|v| {
                v.iter()
                    .skip(top_right.1)
                    .take(1)
                    .all(|ch| *ch == '+' || *ch == '|')
            })
        // top side
        && grid.iter().skip(top_left.0).take(1).all(|v| {
            v.iter()
                .skip(top_left.1)
                .take(top_right.1 - top_left.1 + 1)
                .all(|ch| *ch == '+' || *ch == '-')
        })
        // bottom side
        && grid.iter().skip(bot_left.0).take(1).all(|v| {
            v.iter()
                .skip(bot_left.1)
                .take(bot_right.1 - bot_left.1 + 1)
                .all(|ch| *ch == '+' || *ch == '-')
        })
}
