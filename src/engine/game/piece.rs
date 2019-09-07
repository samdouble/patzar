use std::cmp::PartialEq;
use super::pieces::King;
use super::pieces::Queen;
use super::pieces::Rook;
use super::pieces::Bishop;
use super::pieces::Knight;
use super::pieces::Pawn;

#[derive(Debug)]
pub enum Piece {
    King(King),
    Queen(Queen),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Pawn(Pawn),
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Piece::King(k1), Piece::King(k2)) => k1 == k2,
            (Piece::Queen(q1), Piece::Queen(q2)) => q1 == q2,
            (Piece::Rook(r1), Piece::Rook(r2)) => r1 == r2,
            (Piece::Bishop(b1), Piece::Bishop(b2)) => b1 == b2,
            (Piece::Knight(n1), Piece::Knight(n2)) => n1 == n2,
            (Piece::Pawn(p1), Piece::Pawn(p2)) => p1 == p2,
            _ => false,
        }
    }
}

// TODO tests PartialEq
