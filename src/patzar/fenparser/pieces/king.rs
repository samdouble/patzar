use std::cmp::PartialEq;
use crate::patzar::fenparser::Square;
use super::Color;

#[derive(Debug)]
pub struct King {
    color: Color,
    square: Square,
}

impl King {
    pub fn new(color: Color, square: Square) -> Self {
        Self {
            color,
            square,
        }
    }
}

impl PartialEq for King {
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}
