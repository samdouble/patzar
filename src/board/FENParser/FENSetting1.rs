pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl FENSetting1 {
    pub fn validate(option: &'static str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting1;

    #[test]
    fn validate_initial_board() {
        let valid = FENSetting2::validate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(valid, true);
    }

    #[test]
    fn invalidate_empty() {
        let valid = FENSetting2::validate("");
        assert_eq!(valid, false);
    }
}