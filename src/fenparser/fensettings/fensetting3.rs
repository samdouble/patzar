const NO_POSSIBLE_CASTLING: char = '-';

pub struct FENSetting3 {}

// Third setting: castling options
impl FENSetting3 {
    pub fn validate(setting: &'static str) -> bool {
        if setting == NO_POSSIBLE_CASTLING.to_string() {
            return true;
        }
        let mut options = vec!['K', 'Q', 'k', 'q'];
        let castling_options: Vec<char> = setting.chars().collect();
        for castling_option in castling_options {
            if !options.contains(&castling_option) {
                return false;
            }
            options.retain(|&x| x != castling_option);
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting3;

    #[test]
    fn validate_no_castling_options() {
        let valid = FENSetting3::validate("-");
        assert!(valid);
    }

    #[test]
    fn validate_all_castling_options_in_usual_order() {
        let valid = FENSetting3::validate("KQkq");
        assert!(valid);
    }


    #[test]
    fn validate_only_one_castling_option() {
        let valid = FENSetting3::validate("Q");
        assert!(valid);
    }

    #[test]
    fn invalidate_dash_and_castling_options() {
        let valid = FENSetting3::validate("-KQ");
        assert!(!valid);
    }

    #[test]
    fn validate_all_castling_options_in_unusual_order() {
        let valid = FENSetting3::validate("qKkQ");
        assert!(valid);
    }

    #[test]
    fn invalidate_one_castling_option_repeated_twice() {
        let valid = FENSetting3::validate("qKkQK");
        assert!(!valid);
    }

    #[test]
    fn invalidate_one_castling_option_repeated_thrice() {
        let valid = FENSetting3::validate("qKkQKq");
        assert!(!valid);
    }
}