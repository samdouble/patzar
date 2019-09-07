use crate::patzar::fenparser::Square;
use super::Color;

#[derive(Debug)]
pub struct Bishop {
    color: Color,
    square: Square,
}

impl Bishop {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}

impl PartialEq for Bishop {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}
