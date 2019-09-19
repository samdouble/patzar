use crate::engine::game::Color;
use crate::engine::game::Square;
use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;

#[derive(Debug, Copy, Clone)]
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

impl Movable for Pawn {
    fn get_possible_moves(&self) -> Vec<Move> {
        let moves = Vec::new();
        moves
    }
}
