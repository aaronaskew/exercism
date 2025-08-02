#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            None
        } else {
            Some(Self { rank, file })
        }
    }

    pub fn same_rank(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    pub fn same_file(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    pub fn same_diagonal(&self, other: &ChessPosition) -> bool {
        (self.rank - other.rank).abs() == (self.file - other.file).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.same_rank(&other.0) || self.0.same_file(&other.0) || self.0.same_diagonal(&other.0)
    }
}
