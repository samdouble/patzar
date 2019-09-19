use crate::engine::fenparser::Validatable;
use crate::engine::fenparser::errors::Error;

pub struct FENSetting6 {}

// Sixth setting: number of full moves
impl Validatable<u8, Error> for FENSetting6 {
    fn validate(setting: &str) -> Result<u8, Error> {
        match setting.parse::<u8>() {
            Ok(num_moves) => {
                if num_moves >= 1 {
                    Ok(num_moves)
                } else {
                    Err(Error::InvalidFullMovesSetting)
                }
            },
            Err(_e) => Err(Error::InvalidFullMovesSetting), 
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::fensettings::FENSetting6;
    use crate::engine::fenparser::Validatable;

    #[test]
    fn validate_1() {
        let nb_full_moves = FENSetting6::validate("1");
        let valid = match nb_full_moves {
            Ok(_nb_full_moves) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_30() {
        let nb_full_moves = FENSetting6::validate("30");
        let valid = match nb_full_moves {
            Ok(_nb_full_moves) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn invalidate_0() {
        let nb_full_moves = FENSetting6::validate("0");
        let valid = match nb_full_moves {
            Ok(_nb_full_moves) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_negative() {
        let nb_full_moves = FENSetting6::validate("-1");
        let valid = match nb_full_moves {
            Ok(_nb_full_moves) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_decimal() {
        let nb_full_moves = FENSetting6::validate("2.0");
        let valid = match nb_full_moves {
            Ok(_nb_full_moves) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}