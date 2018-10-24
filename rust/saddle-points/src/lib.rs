pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output = Vec::new();

    for (row, v) in input.iter().enumerate() {
        for (col, val) in v.iter().enumerate() {
            if v.iter().all(|x| x <= val) && (0..input.len()).all(|x| input[x][col] >= *val) {
                output.push((row, col));
            }
        }
    }
    output
}
