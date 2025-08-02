pub fn get_diamond(c: char) -> Vec<String> {
    let dim = ((c as u8 - b'A') * 2 + 1) as usize;

    let mut diamond = vec![vec![' '; dim]; dim];

    diamond
        .iter_mut()
        .enumerate()
        .map(|(j, v)| {
            v.iter_mut()
                .enumerate()
                .map(move |(i, _)| {
                    let center = dim / 2;
                    let i_dist_from_center = (i as i32 - center as i32).unsigned_abs() as usize;
                    let j_dist_from_center = (j as i32 - center as i32).unsigned_abs() as usize;

                    let letter = match j {
                        j if j < center || j == 0 => (b'A' + j as u8) as char,
                        j if j == center => c,
                        j if j > center => (c as u8 - (j - center) as u8) as char,
                        _ => '\0',
                    };

                    if i_dist_from_center
                        == (j_dist_from_center as i32 - center as i32).unsigned_abs() as usize
                    {
                        letter
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
