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

impl PartialEq for Rook {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
