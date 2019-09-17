use crate::engine::fenparser::Validatable;

const ROWS_SEPARATOR: char = '/';

pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl Validatable for FENSetting1 {
    fn validate(setting: &str) -> bool {
        true
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