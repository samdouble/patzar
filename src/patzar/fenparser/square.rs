use std::cmp::PartialEq;
use regex::Regex;
use crate::patzar::fenparser::fenparsable::FENParsable;
use super::piece::Piece;

#[derive(Debug)]
pub struct Square {
    row: u8,
    col: char,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(row: u8, col: char) -> Self {
        Self {
            row,
            col,
            piece: None,
        }
    }

    pub fn assign_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn get_row(&self) -> u8 {
        self.row
    }
}

impl FENParsable<Self> for Square {
    fn from_FEN_string(fen_string: &str) -> Result<Self, ()> {
        let square_regex: Regex = Regex::new(r"^[a-h][1-8]$").unwrap();
        match square_regex.is_match(fen_string) {
            true => Ok(
                Self::new(
                    fen_string
                        .chars().nth(1).unwrap().to_string()
                        .parse::<u8>().unwrap(),
                    fen_string.chars().nth(0).unwrap(), 
                )
            ),
            false => Err(()),
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
    use crate::patzar::fenparser::fenparsable::FENParsable;
    use super::Square;

    #[test]
    fn validate_a1() {
        let square = Square::from_FEN_string("a1").unwrap();
        assert_eq!(square, Square::new(1, 'a'));
    }

    #[test]
    fn validate_a8() {
        let square = Square::from_FEN_string("a8").unwrap();
        assert_eq!(square, Square::new(8, 'a'));
    }

    #[test]
    fn validate_h1() {
        let square = Square::from_FEN_string("h1").unwrap();
        assert_eq!(square, Square::new(1, 'h'));
    }

    #[test]
    fn validate_h8() {
        let square = Square::from_FEN_string("h8").unwrap();
        assert_eq!(square, Square::new(8, 'h'));
    }

    #[test]
    fn validate_e5() {
        let square = Square::from_FEN_string("e5").unwrap();
        assert_eq!(square, Square::new(5, 'e'));
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