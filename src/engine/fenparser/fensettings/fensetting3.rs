use crate::engine::fenparser::Validatable;
use crate::engine::fenparser::errors::Error;
use crate::engine::game::CastlingOption;
use crate::engine::game::Color;

const NO_POSSIBLE_CASTLING: char = '-';

pub struct FENSetting3 {}

// Third setting: castling options
impl Validatable<Option<Vec<CastlingOption>>, Error> for FENSetting3 {
    fn validate(setting: &str) -> Result<Option<Vec<CastlingOption>>, Error> {
        if setting == NO_POSSIBLE_CASTLING.to_string() {
            return Ok(None);
        }
        let mut options = Vec::new();
        let castling_options: Vec<char> = setting.chars().collect();
        for castling_option in castling_options {
            let option = match castling_option {
                'K' => CastlingOption::KingSide(Color::White),
                'Q' => CastlingOption::QueenSide(Color::White),
                'k' => CastlingOption::KingSide(Color::Black),
                'q' => CastlingOption::QueenSide(Color::Black),
                _ => return Err(Error::InvalidCastlingOptionSetting),
            };
            if !options.contains(&option) {
                options.push(option);
            } else {
                return Err(Error::CastlingOptionSpecifiedTwice);
            }
        }
        Ok(Some(options))
    }
}
/*
#[cfg(test)]
mod tests {
    use crate::engine::fenparser::fensettings::FENSetting3;
    use crate::engine::fenparser::Validatable;

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
}*/