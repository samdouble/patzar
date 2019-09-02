use super::super::square::Square;

pub Rook {
    square: Square,
}

impl Rook {
    pub fn new(square: &Square) -> Self {
        Self {
            square,
        }
    }
}
