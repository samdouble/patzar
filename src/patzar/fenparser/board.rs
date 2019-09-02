use crate::patzar::fenparser::validatable::Validatable;
use super::row::Row;

const NUMBER_OF_ROWS: u8 = 8;
const ROWS_SEPARATOR: char = '/';

pub struct Board {}

impl Validatable for Board {
    fn validate(fen_board: &str) -> bool {
        let rows = fen_board.split(ROWS_SEPARATOR);

        let mut rows_count = 0;
        for (_i, row) in rows.enumerate() {
            if !Row::validate(row) {
                return false;
            }
            // ...
            rows_count += 1;
        };
        if rows_count != NUMBER_OF_ROWS {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::patzar::fenparser::validatable::Validatable;
    use super::Board;
    
    #[test]
    fn initial_configuration_is_valid() {
        let valid = Board::validate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }
}
