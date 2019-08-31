use core::str::FromStr;

pub enum Piece {
    K, k, Q, q, R, r, B, b, N, n, P, p,
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
            "K" => Ok(Piece::K),
            "k" => Ok(Piece::k),
            "Q" => Ok(Piece::Q),
            "q" => Ok(Piece::q),
            "R" => Ok(Piece::R),
            "r" => Ok(Piece::r),
            "B" => Ok(Piece::B),
            "b" => Ok(Piece::b),
            "N" => Ok(Piece::N),
            "n" => Ok(Piece::n),
            "P" => Ok(Piece::P),
            "p" => Ok(Piece::p),
            _ => Err(()),
        }
    }
}