use std::cmp::PartialEq;
use crate::patzar::fenparser::fenparsable::FENParsable;
use super::pieces::Color;
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

impl FENParsable<Self> for Piece {
    fn from_FEN_string(fen_string: &str) -> Result<Self, ()> {
        match fen_string {
            "K" => Ok(Piece::King(King::new(Color::White))),
            "k" => Ok(Piece::King(King::new(Color::Black))),
            "Q" => Ok(Piece::Queen(Queen::new(Color::White))),
            "q" => Ok(Piece::Queen(Queen::new(Color::Black))),
            "R" => Ok(Piece::Rook(Rook::new(Color::White))),
            "r" => Ok(Piece::Rook(Rook::new(Color::Black))),
            "B" => Ok(Piece::Bishop(Bishop::new(Color::White))),
            "b" => Ok(Piece::Bishop(Bishop::new(Color::Black))),
            "N" => Ok(Piece::Knight(Knight::new(Color::White))),
            "n" => Ok(Piece::Knight(Knight::new(Color::Black))),
            "P" => Ok(Piece::Pawn(Pawn::new(Color::White))),
            "p" => Ok(Piece::Pawn(Pawn::new(Color::Black))),
            _ => Err(()),
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::patzar::fenparser::fenparsable::FENParsable;
    use super::Color;
    use super::Piece;
    use super::King;
    use super::Queen;
    use super::Rook;
    use super::Bishop;
    use super::Knight;
    use super::Pawn;

    #[test]
    fn create_white_king_from_fen() {
        let king = Piece::from_FEN_string("K").unwrap();
        assert_eq!(king, Piece::King(King::new(Color::White)));
    }

    #[test]
    fn create_black_king_from_fen() {
        let king = Piece::from_FEN_string("k").unwrap();
        assert_eq!(king, Piece::King(King::new(Color::Black)));
    }

    #[test]
    fn create_white_queen_from_fen() {
        let queen = Piece::from_FEN_string("Q").unwrap();
        assert_eq!(queen, Piece::Queen(Queen::new(Color::White)));
    }

    #[test]
    fn create_black_queen_from_fen() {
        let queen = Piece::from_FEN_string("q").unwrap();
        assert_eq!(queen, Piece::Queen(Queen::new(Color::Black)));
    }

    #[test]
    fn create_white_rook_from_fen() {
        let rook = Piece::from_FEN_string("R").unwrap();
        assert_eq!(rook, Piece::Rook(Rook::new(Color::White)));
    }

    #[test]
    fn create_black_rook_from_fen() {
        let rook = Piece::from_FEN_string("r").unwrap();
        assert_eq!(rook, Piece::Rook(Rook::new(Color::Black)));
    }

    #[test]
    fn create_white_bishop_from_fen() {
        let bishop = Piece::from_FEN_string("B").unwrap();
        assert_eq!(bishop, Piece::Bishop(Bishop::new(Color::White)));
    }

    #[test]
    fn create_black_bishop_from_fen() {
        let bishop = Piece::from_FEN_string("b").unwrap();
        assert_eq!(bishop, Piece::Bishop(Bishop::new(Color::Black)));
    }

    #[test]
    fn create_white_knight_from_fen() {
        let knight = Piece::from_FEN_string("N").unwrap();
        assert_eq!(knight, Piece::Knight(Knight::new(Color::White)));
    }

    #[test]
    fn create_black_knight_from_fen() {
        let knight = Piece::from_FEN_string("n").unwrap();
        assert_eq!(knight, Piece::Knight(Knight::new(Color::Black)));
    }

    #[test]
    fn create_white_pawn_from_fen() {
        let pawn = Piece::from_FEN_string("P").unwrap();
        assert_eq!(pawn, Piece::Pawn(Pawn::new(Color::White)));
    }

    #[test]
    fn create_black_pawn_from_fen() {
        let pawn = Piece::from_FEN_string("p").unwrap();
        assert_eq!(pawn, Piece::Pawn(Pawn::new(Color::Black)));
    }
    
    #[test]
    fn create_invalid_piece_from_fen() {
        let piece = Piece::from_FEN_string("w");
        let valid = match piece {
            Ok(_piece) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}
