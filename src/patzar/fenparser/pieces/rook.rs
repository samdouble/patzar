use super::Color;

#[derive(Debug)]
pub struct Rook {
    color: Color,
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
