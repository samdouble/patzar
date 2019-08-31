use super::super::board::*;

pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl FENSetting1 {
    pub fn validate(setting: &'static str) -> bool {
        Board::validate(setting)
    }
}

#[cfg(test)]
mod tests {
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