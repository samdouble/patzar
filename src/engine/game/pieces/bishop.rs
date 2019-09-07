use crate::engine::game::Color;
use crate::engine::game::Square;

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
