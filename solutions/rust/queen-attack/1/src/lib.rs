#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        [rank, file].iter().all(|i| (0..8).contains(i)).then_some(Self(rank, file)).or(None)
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        use std::f64::consts::PI;
        f64::atan2((self.0.0 - other.0.0) as f64, (self.0.1 - other.0.1) as f64) / PI * 180f64 % 45f64 == 0.0
    }
}
