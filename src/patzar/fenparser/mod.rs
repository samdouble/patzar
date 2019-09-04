use crate::patzar::fenparser::validatable::Validatable;
mod fensettings;
use fensettings::fensetting1::FENSetting1;
use fensettings::fensetting2::FENSetting2;
use fensettings::fensetting3::FENSetting3;
use fensettings::fensetting4::FENSetting4;
use fensettings::fensetting5::FENSetting5;
use fensettings::fensetting6::FENSetting6;
pub mod board;
pub mod row;
pub mod square;
pub mod piece;
pub mod pieces;
pub mod fenparsable;
pub mod validatable;

const NUMBER_OF_SETTINGS: u8 = 6;
const SETTINGS_SEPARATOR: char = ' ';

pub struct FENParser {}

impl Validatable for FENParser {
    fn validate(fen_string: &str) -> bool {
        let settings = fen_string.split(SETTINGS_SEPARATOR);

        let mut settings_count = 0;
        for (i, setting) in settings.enumerate() {
            let setting_valid = match i {
                0 => FENSetting1::validate(setting),
                1 => FENSetting2::validate(setting),
                2 => FENSetting3::validate(setting),
                3 => FENSetting4::validate(setting),
                4 => FENSetting5::validate(setting),
                5 => FENSetting6::validate(setting),
                _ => false,
            };
            if !setting_valid {
                return false;
            }
            settings_count += 1;
        };
        if settings_count != NUMBER_OF_SETTINGS {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::patzar::fenparser::validatable::Validatable;
    use crate::patzar::game::Game;
    use super::FENParser;

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert!(FENParser::validate(initial_configuration));
    }
}