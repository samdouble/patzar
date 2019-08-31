pub struct FENSetting5 {}

// Fifth setting: half-move counter
impl FENSetting5 {
    pub fn validate(setting: &'static str) -> bool {
        match setting.parse::<u32>() {
            Ok(num_moves) => true,
            Err(e) => false, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting5;

    #[test]
    fn validate_1() {
        let valid = FENSetting5::validate("1");
        assert!(valid);
    }

    #[test]
    fn validate_30() {
        let valid = FENSetting5::validate("30");
        assert!(valid);
    }

    #[test]
    fn validate_0() {
        let valid = FENSetting5::validate("0");
        assert!(valid);
    }

    #[test]
    fn invalidate_negative() {
        let valid = FENSetting5::validate("-1");
        assert!(!valid);
    }

    #[test]
    fn invalidate_decimal() {
        let valid = FENSetting5::validate("2.0");
        assert!(!valid);
    }
}