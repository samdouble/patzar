use super::Color;

#[derive(Debug)]
pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
