use std::cmp::PartialEq;
use crate::engine::game::Color;

pub enum CastlingOption {
    KingSide(Color),
    QueenSide(Color),
}

impl PartialEq for CastlingOption {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CastlingOption::KingSide(color1), CastlingOption::KingSide(color2)) => color1 == color2,
            (CastlingOption::QueenSide(color1), CastlingOption::QueenSide(color2)) => color1 == color2,
            _ => false,
        }
    }
}
