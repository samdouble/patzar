use super::Color;

#[derive(Debug)]
pub struct Pawn {
    color: Color,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
