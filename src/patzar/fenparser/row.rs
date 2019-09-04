use crate::patzar::fenparser::fenparsable::FENParsable;
use super::square::Square;
use super::piece::Piece;

pub struct Row {}

const NUMBER_OF_COLS: usize = 8;

impl FENParsable<Self> for Row {
    fn from_FEN_string(fen_string: &str) -> Result<Self, ()> {
        let squares: Vec<char> = fen_string.chars().collect();
        let mut vecSquares: Vec<Square> = Vec::new();
        for square in squares {
            let square = &square.to_string();
            vecSquares.append(
                match square.parse::<u8>() {
                    Ok(num_squares) => {
                        let mut empty_squares = Vec::new();
                        for i in 1..num_squares {
                            empty_squares.push(Square::new());
                        }
                        &mut empty_squares
                    },
                    Err(_err) => {
                        match Piece::from_FEN_string(&square) {
                            Ok(piece) => {
                                let mut square = Square::new();
                                square.assign_piece(piece);
                                &mut vec![square]
                            },
                            Err(_err) => return Err(()),
                        }
                    },
                }
            )
        };
        if vecSquares.len() != NUMBER_OF_COLS {
            return Err(());
        }
        Ok(vecSquares)
    }
}

// TODO tests
