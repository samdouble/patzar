use crate::patzar::fenparser::fenparsable::FENParsable;
use super::piece::Piece;
use super::square::Square;

const NUMBER_OF_ROWS: usize = 8;
const NUMBER_OF_COLS: usize = 8;
const ROWS_SEPARATOR: char = '/';

pub type Board = Vec<Vec<Square>>;

impl FENParsable<Self> for Board {
    fn from_FEN_string(fen_string: &str) -> Result<Self, ()> {
        let mut vec_rows: Self = Vec::new();
        let mut row_number = 0;
        for row in fen_string.split(ROWS_SEPARATOR) {
            let squares: Vec<char> = row.chars().collect();
            let mut vec_squares: Vec<Square> = Vec::new();
            let mut col_number = 0;
            for square in squares {
                let square = square.to_string();
                match square.parse::<usize>() {
                    Ok(num_squares) => {
                        for _ in 0..num_squares {
                            vec_squares.push(Square::new(row_number, col_number));
                            col_number += 1;
                        }
                    },
                    Err(_err) => {
                        match Piece::from_FEN_string(&square) {
                            Ok(piece) => {
                                let mut square = Square::new(row_number, col_number);
                                square.assign_piece(piece);
                                col_number += 1;
                                vec_squares.push(square);
                            },
                            Err(_err) => return Err(()),
                        }
                    },
                }
            };
            if col_number != NUMBER_OF_COLS {
                return Err(());
            }
            // TODO more conditions ...
            vec_rows.push(vec_squares);
            row_number += 1;
        };
        if row_number != NUMBER_OF_ROWS {
            return Err(());
        }
        Ok(vec_rows)
    }
}

#[cfg(test)]
mod tests {
    /*use super::Board;
    
    #[test]
    fn initial_configuration_is_valid() {
        let valid = Board::from_FEN_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }

    // TODO more tests*/
}
