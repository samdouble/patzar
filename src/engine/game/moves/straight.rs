use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;

pub trait MovableStraight: Movable {
    fn get_possible_moves(&self) -> Vec<Move>;
}
