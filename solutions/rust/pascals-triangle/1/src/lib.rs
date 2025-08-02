pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let row_count = self.0;

        let mut triangle: Vec<Vec<u32>> = Vec::new();

        if row_count == 0 {
            return triangle;
        }

        // The first row is [1]
        triangle.push(vec![1]);

        for i in 1..row_count {
            let mut row: Vec<u32> = vec![1]; 

            for j in 1..i {
                let left = triangle[i as usize - 1][j as usize - 1];
                let right = triangle[i as usize - 1][j as usize];
                row.push(left + right);
            }

            row.push(1); 
            triangle.push(row);
        }

        triangle
    }
}
