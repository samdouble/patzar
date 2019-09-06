use super::Color;

#[derive(Debug)]
pub struct Bishop {
    color: Color,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}

impl PartialEq for Bishop {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
