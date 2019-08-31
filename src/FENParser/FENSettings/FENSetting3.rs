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
    fn validate_KQkq() {
        let valid = FENSetting3::validate("KQkq");
        assert!(valid);
    }


    #[test]
    fn validate_Q() {
        let valid = FENSetting3::validate("Q");
        assert!(valid);
    }

    #[test]
    fn invalidate_dash_and_castling_options() {
        let valid = FENSetting3::validate("-KQ");
        assert!(!valid);
    }

    #[test]
    fn validate_qKkQ() {
        let valid = FENSetting3::validate("qKkQ");
        assert!(valid);
    }

    #[test]
    fn invalidate_qKkQK() {
        let valid = FENSetting3::validate("qKkQK");
        assert!(!valid);
    }

    #[test]
    fn invalidate_qKkQKq() {
        let valid = FENSetting3::validate("qKkQKq");
        assert!(!valid);
    }
}