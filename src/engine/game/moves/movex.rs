use crate::engine::game::Square;

#[derive(Copy, Clone)]
pub struct Move {
    from: Square,
    to: Square,
}
