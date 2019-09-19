use crate::engine::game::Color;
use crate::engine::game::Square;
use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;

#[derive(Debug, Copy, Clone)]
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

impl Movable for Bishop {
    fn get_possible_moves(&self) -> Vec<Move> {
        let moves = Vec::new();
        moves
    }
}
