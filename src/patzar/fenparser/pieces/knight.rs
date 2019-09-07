use crate::patzar::fenparser::Square;
use super::Color;

#[derive(Debug)]
pub struct Knight {
    color: Color,
    square: Square,
}

impl Knight {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}

impl PartialEq for Knight {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}
