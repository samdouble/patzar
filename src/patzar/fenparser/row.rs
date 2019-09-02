use crate::patzar::fenparser::validatable::Validatable;
use super::piece::Piece;

pub struct Row {}

const NUMBER_OF_COLS: u8 = 8;

impl Validatable for Row {
    fn validate(fen_row: &str) -> bool {
        let squares: Vec<char> = fen_row.chars().collect();
        let mut squares_in_row_count: u8 = 0;
        for square in squares {
            squares_in_row_count += match square.to_string().parse::<u8>() {
                Ok(num_squares) => num_squares,
                Err(_e) => {
                    if !Piece::validate(&square.to_string()) {
                        return false;
                    }
                    1
                },
            }
        };
        if squares_in_row_count != NUMBER_OF_COLS {
            return false;
        }
        true
    }
}

// TODO tests
