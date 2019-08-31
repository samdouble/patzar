pub struct FENSetting6 {}

// Sixth setting: number of full moves
impl FENSetting6 {
    pub fn validate(setting: &'static str) -> bool {
        match setting.parse::<u32>() {
            Ok(num_moves) => num_moves >= 1,
            Err(e) => false, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting6;

    #[test]
    fn validate_1() {
        let valid = FENSetting6::validate("1");
        assert!(valid);
    }

    #[test]
    fn validate_30() {
        let valid = FENSetting6::validate("30");
        assert!(valid);
    }

    #[test]
    fn invalidate_0() {
        let valid = FENSetting6::validate("0");
        assert!(!valid);
    }

    #[test]
    fn invalidate_negative() {
        let valid = FENSetting6::validate("-1");
        assert!(!valid);
    }

    #[test]
    fn invalidate_decimal() {
        let valid = FENSetting6::validate("2.0");
        assert!(!valid);
    }
}