use crate::engine::game::Validatable;

pub struct FENSetting2 {}

// Second setting: next player to move
impl Validatable for FENSetting2 {
    fn validate(setting: &str) -> bool {
        setting == "w" || setting == "b"
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::fensettings::FENSetting2;
    use crate::engine::game::Validatable;

    #[test]
    fn validate_white() {
        let valid = FENSetting2::validate("w");
        assert!(valid);
    }

    #[test]
    fn validate_black() {
        let valid = FENSetting2::validate("b");
        assert!(valid);
    }

    #[test]
    fn invalidate_wb() {
        let valid = FENSetting2::validate("wb");
        assert!(!valid);
    }

    #[test]
    fn invalidate_space() {
        let valid = FENSetting2::validate(" ");
        assert!(!valid);
    }
}