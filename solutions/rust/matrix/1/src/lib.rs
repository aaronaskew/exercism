pub struct Matrix {
    matrix: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let matrix = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        Self { matrix }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if (0..self.matrix.len()).contains(&(row_no - 1)) {
            Some(self.matrix[row_no - 1].to_vec())
        } else {
            None
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if (0..self.matrix[0].len()).contains(&(col_no - 1)) {
            Some(
                (0..self.matrix.len())
                    .map(|row_no| self.matrix[row_no][col_no - 1])
                    .collect::<Vec<u32>>(),
            )
        } else {
            None
        }
    }
}
