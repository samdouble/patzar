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

impl PartialEq for Knight {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
