use crate::patzar::fenparser::Square;
use super::Color;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
    square: Square,
}

impl Pawn {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}

impl PartialEq for Pawn {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}
