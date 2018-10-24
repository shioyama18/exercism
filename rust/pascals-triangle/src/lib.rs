pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(PascalsTriangle::calc).collect()
    }

    fn calc(row: u32) -> Vec<u32> {
        let mut output = vec![1];
        for i in 1..row + 1 {
            if let Some(&last) = output.last() {
                output.push(last * (row + 1 - i) / i);
            }
        }
        output
    }
}
