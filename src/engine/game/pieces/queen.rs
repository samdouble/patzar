use crate::engine::game::Color;
use crate::engine::game::Square;
use crate::engine::game::moves::Movable;
use crate::engine::game::moves::MovableStraight;
use crate::engine::game::moves::Move;

#[derive(Debug, Clone)]
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

impl Movable for Queen {
    fn get_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        moves.append(&mut { <Self as MovableStraight>::get_possible_moves(&self) });
        moves
    }
}

impl MovableStraight for Queen {
    fn get_possible_moves(&self) -> Vec<Move> {
        let moves = Vec::new();
        moves
    }
}
