use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;
use crate::engine::game::pieces::King;
use crate::engine::game::pieces::Queen;
use crate::engine::game::pieces::Rook;
use crate::engine::game::pieces::Bishop;
use crate::engine::game::pieces::Knight;
use crate::engine::game::pieces::Pawn;

#[derive(Debug, Clone)]
pub enum Piece {
    King(King),
    Queen(Queen),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Pawn(Pawn),
}

impl Movable for Piece {
    fn get_possible_moves(&self) -> Vec<Move> {
        match self {
            Piece::King(k) => k.get_possible_moves(),
            Piece::Queen(q) => q.get_possible_moves(),
            Piece::Rook(r) => r.get_possible_moves(),
            Piece::Bishop(b) => b.get_possible_moves(),
            Piece::Knight(n) => n.get_possible_moves(),
            Piece::Pawn(p) => p.get_possible_moves(),
        }
    }
}
