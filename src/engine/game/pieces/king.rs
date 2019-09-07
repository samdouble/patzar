use crate::engine::game::Color;
use crate::engine::game::Square;

#[derive(Debug)]
pub struct King {
    color: Color,
    square: Square,
}

impl King {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}
