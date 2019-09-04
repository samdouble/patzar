use super::Color;

#[derive(Debug)]
pub struct Queen {
    color: Color,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
