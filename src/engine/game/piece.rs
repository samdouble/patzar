use crate::engine::game::pieces::King;
use crate::engine::game::pieces::Queen;
use crate::engine::game::pieces::Rook;
use crate::engine::game::pieces::Bishop;
use crate::engine::game::pieces::Knight;
use crate::engine::game::pieces::Pawn;

#[derive(Debug)]
pub enum Piece {
    King(King),
    Queen(Queen),
    Rook(Rook),
    Bishop(Bishop),
    Knight(Knight),
    Pawn(Pawn),
}
