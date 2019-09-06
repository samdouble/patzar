use crate::patzar::fenparser::fenparsable::FENParsable;
use crate::patzar::fenparser::validatable::Validatable;
use super::super::board::Board;

pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl Validatable for FENSetting1 {
    fn validate(setting: &str) -> bool {
        match Board::from_FEN_string(setting) {
            Ok(_board) => true,
            Err(_err) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::patzar::fenparser::validatable::Validatable;
    use super::FENSetting1;

    #[test]
    fn validate_initial_board() {
        let valid = FENSetting1::validate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }

    #[test]
    fn invalidate_empty() {
        let valid = FENSetting1::validate("");
        assert!(!valid);
    }
}