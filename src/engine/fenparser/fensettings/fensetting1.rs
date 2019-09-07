use crate::engine::fenparser::FENParsable;
use crate::engine::fenparser::Validatable;
use crate::engine::game::Board;

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
    use crate::engine::fenparser::fensettings::FENSetting1;
    use crate::engine::fenparser::Validatable;

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