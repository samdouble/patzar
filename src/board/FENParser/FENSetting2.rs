pub struct FENSetting2 {}

// Second setting: next player to move
impl FENSetting2 {
    pub fn validate(setting: &'static str) -> bool {
        setting == "w" || setting == "b"
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting2;

    #[test]
    fn validate_white() {
        let valid = FENSetting2::validate("w");
        assert_eq!(valid, true);
    }

    #[test]
    fn validate_black() {
        let valid = FENSetting2::validate("b");
        assert_eq!(valid, true);
    }

    #[test]
    fn invalidate_wb() {
        let valid = FENSetting2::validate("wb");
        assert_eq!(valid, false);
    }

    #[test]
    fn invalidate_space() {
        let valid = FENSetting2::validate(" ");
        assert_eq!(valid, false);
    }
}