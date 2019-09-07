use crate::engine::game::Color;
use crate::engine::game::Square;

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
