use crate::engine::fenparser::Validatable;
use crate::engine::fenparser::errors::Error;
use crate::engine::game::Color;

pub struct FENSetting2 {}

// Second setting: next player to move
impl Validatable<Color, Error> for FENSetting2 {
    fn validate(setting: &str) -> Result<Color, Error> {
        match setting {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(Error::InvalidNextPlayerSetting),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::errors::Error;
    use crate::engine::fenparser::fensettings::FENSetting2;
    use crate::engine::fenparser::Validatable;
    use crate::engine::game::Color;

    #[test]
    fn validate_white() {
        let color: Result<Color, Error> = FENSetting2::validate("w");
        let valid = match color {
            Ok(_color) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_black() {
        let color = FENSetting2::validate("b");
        let valid = match color {
            Ok(_color) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn invalidate_wb() {
        let color = FENSetting2::validate("wb");
        let valid = match color {
            Ok(_color) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }

    #[test]
    fn invalidate_space() {
        let color = FENSetting2::validate(" ");
        let valid = match color {
            Ok(_color) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}