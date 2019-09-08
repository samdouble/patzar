use crate::engine::game::moves::Move;

pub trait Movable {
    fn get_possible_moves(&self) -> Vec<Move>;
}