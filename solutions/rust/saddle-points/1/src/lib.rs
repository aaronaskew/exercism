pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();

    for i in 0..input[0].len() {
        for j in 0..input.len() {
            let row = input[j].clone();
            let col = input.iter().map(|x| x[i]).collect::<Vec<u64>>();
            if input[j][i] == *col.iter().min().unwrap()
                && input[j][i] == *row.iter().max().unwrap()
            {
                saddle_points.push((j, i));
            }
        }
    }

    saddle_points
}
