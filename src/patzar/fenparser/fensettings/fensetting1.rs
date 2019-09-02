use crate::patzar::fenparser::validatable::Validatable;
use super::super::board::Board;

pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl Validatable for FENSetting1 {
    fn validate(setting: &str) -> bool {
        Board::validate(setting)
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