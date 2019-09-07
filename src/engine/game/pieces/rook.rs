use crate::engine::game::Color;
use crate::engine::game::Square;

#[derive(Debug)]
pub struct Rook {
    color: Color,
    square: Square,
}

impl Rook {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}
