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

impl PartialEq for Queen {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
