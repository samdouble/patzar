use super::Color;

#[derive(Debug)]
pub struct King {
    color: Color,
}

impl King {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
