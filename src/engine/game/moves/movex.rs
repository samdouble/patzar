use std::cmp::PartialEq;
use crate::engine::game::Square;

#[derive(Debug, Copy, Clone)]
pub struct Move {
    from: Square,
    to: Square,
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}
