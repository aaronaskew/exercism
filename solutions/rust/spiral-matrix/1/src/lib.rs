enum Direction {
    N,
    S,
    E,
    W,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut spiral = vec![vec![0; size]; size];

    if size == 0 {
        return spiral;
    }

    let mut direction = Direction::E;

    let mut i = 0;
    let mut j = 0;
    let mut val = 1;

    loop {
        spiral[j][i] = val;
        val += 1;

        if val as usize > size * size {
            break;
        }

        loop {
            match direction {
                Direction::N => {
                    if j > 0 && spiral[j - 1][i] == 0 {
                        j -= 1;
                        break;
                    } else {
                        direction = Direction::E;
                    }
                }
                Direction::S => {
                    if j + 1 < size && spiral[j + 1][i] == 0 {
                        j += 1;
                        break;
                    } else {
                        direction = Direction::W;
                    }
                }
                Direction::E => {
                    if i + 1 < size && spiral[j][i + 1] == 0 {
                        i += 1;
                        break;
                    } else {
                        direction = Direction::S;
                    }
                }
                Direction::W => {
                    if i > 0 && spiral[j][i - 1] == 0 {
                        i -= 1;
                        break;
                    } else {
                        direction = Direction::N;
                    }
                }
            }
        }
    }

    spiral
}
