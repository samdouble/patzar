use crate::engine::fenparser::Validatable;
use crate::engine::fenparser::errors::Error;

pub struct FENSetting5 {}

// Fifth setting: half-move counter
impl Validatable<u8, Error> for FENSetting5 {
    fn validate(setting: &str) -> Result<u8, Error> {
        match setting.parse::<u8>() {
            Ok(num_moves) => Ok(num_moves),
            Err(_e) => Err(Error::InvalidHalfMovesSetting), 
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::fensettings::FENSetting5;
    use crate::engine::fenparser::Validatable;

    #[test]
    fn validate_1() {
        let nb_half_moves = FENSetting5::validate("1");
        let valid = match nb_half_moves {
            Ok(_nb_half_moves) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_30() {
        let nb_half_moves = FENSetting5::validate("30");
        let valid = match nb_half_moves {
            Ok(_nb_half_moves) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_0() {
        let nb_half_moves = FENSetting5::validate("0");
        let valid = match nb_half_moves {
            Ok(_nb_half_moves) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn invalidate_negative() {
        let nb_half_moves = FENSetting5::validate("-1");
        let valid = match nb_half_moves {
            Ok(_nb_half_moves) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_decimal() {
        let nb_half_moves = FENSetting5::validate("2.0");
        let valid = match nb_half_moves {
            Ok(_nb_half_moves) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}