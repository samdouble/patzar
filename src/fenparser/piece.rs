use core::str::FromStr;

pub enum Piece {
    WhiteKing,
    BlackKing,
    WhiteQueen,
    BlackQueen,
    WhiteRook,
    BlackRook,
    WhiteBishop,
    BlackBishop,
    WhiteKnight,
    BlackKnight,
    WhitePawn,
    BlackPawn,
}

impl Piece {
    pub fn validate(fen_piece: &str) -> bool {
        match fen_piece.parse::<Piece>() {
            Ok(_piece) => true,
            Err(_e) => false,
        }
    }
}

impl FromStr for Piece {
    type Err = ();

    fn from_str(s: &str) -> Result<Piece, ()> {
        match s {
            "K" => Ok(Piece::WhiteKing),
            "k" => Ok(Piece::BlackKing),
            "Q" => Ok(Piece::WhiteQueen),
            "q" => Ok(Piece::BlackQueen),
            "R" => Ok(Piece::WhiteRook),
            "r" => Ok(Piece::BlackRook),
            "B" => Ok(Piece::WhiteBishop),
            "b" => Ok(Piece::BlackBishop),
            "N" => Ok(Piece::WhiteKnight),
            "n" => Ok(Piece::BlackKnight),
            "P" => Ok(Piece::WhitePawn),
            "p" => Ok(Piece::BlackPawn),
            _ => Err(()),
        }
    }
}