use crate::engine::game::Square;

pub struct Move<'a> {
    from: &'a Square,
    to: &'a Square,
}
