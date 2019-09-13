use std::cmp::PartialEq;
use regex::Regex;
use crate::engine::fenparser::errors::Error;
use crate::engine::fenparser::FENParsable;

#[derive(Debug, Copy, Clone)]
pub struct Square {
    row: usize,
    col: usize,
}

impl Square {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }
}

impl FENParsable<Self, Error> for Square {
    fn from_FEN_string(fen_string: &str) -> Result<Self, Error> {
        let square_regex: Regex = Regex::new(r"^[a-h][1-8]$").unwrap();
        match square_regex.is_match(fen_string) {
            true => {
                let row_index = fen_string
                    .chars().nth(1).unwrap().to_string()
                    .parse::<usize>().unwrap() - 1;
                let col_index = match fen_string.chars().nth(0).unwrap() {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => 0,
                };
                Ok(Self::new(row_index, col_index))
            },
            false => Err(Error::InvalidSquareRepresentation),
        }
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::FENParsable;
    use crate::engine::game::Square;

    #[test]
    fn validate_a1() {
        let square = Square::from_FEN_string("a1").unwrap();
        assert_eq!(square, Square::new(0, 0));
    }

    #[test]
    fn validate_a8() {
        let square = Square::from_FEN_string("a8").unwrap();
        assert_eq!(square, Square::new(7, 0));
    }

    #[test]
    fn validate_h1() {
        let square = Square::from_FEN_string("h1").unwrap();
        assert_eq!(square, Square::new(0, 7));
    }

    #[test]
    fn validate_h8() {
        let square = Square::from_FEN_string("h8").unwrap();
        assert_eq!(square, Square::new(7, 7));
    }

    #[test]
    fn validate_e5() {
        let square = Square::from_FEN_string("e5").unwrap();
        assert_eq!(square, Square::new(4, 4));
    }

    #[test]
    fn invalidate_wrong_order_colrow() {
        let square = Square::from_FEN_string("5e");
        let valid = match square {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_capital_letter_for_col() {
        let square = Square::from_FEN_string("E5");
        let valid = match square {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_out_of_bounds_col() {
        let square = Square::from_FEN_string("i1");
        let valid = match square {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_out_of_bounds_row() {
        let square = Square::from_FEN_string("a9");
        let valid = match square {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}