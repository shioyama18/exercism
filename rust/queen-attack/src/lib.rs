#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0...7, 0...7) => Some(ChessPosition { rank, file }),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_diff = self.0.rank - other.0.rank;
        let file_diff = self.0.file - other.0.file;
        match (rank_diff, file_diff) {
            (0, _) => true,
            (_, 0) => true,
            (r, f) if r.abs() == f.abs() => true,
            _ => false
        }
    }
}
