use std::cmp::PartialEq;
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

impl PartialEq for King {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
