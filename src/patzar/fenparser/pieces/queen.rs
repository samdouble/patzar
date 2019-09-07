use crate::patzar::fenparser::Square;
use super::Color;

#[derive(Debug)]
pub struct Queen {
    color: Color,
    square: Square,
}

impl Queen {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}

impl PartialEq for Queen {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}
