mod fensettings;
use fensettings::fensetting1::*;
use fensettings::fensetting2::*;
use fensettings::fensetting3::*;
use fensettings::fensetting4::*;
use fensettings::fensetting5::*;
use fensettings::fensetting6::*;
pub mod game;
pub mod board;
pub mod row;
pub mod square;
pub mod piece;

const NUMBER_OF_SETTINGS: u8 = 6;
const SETTINGS_SEPARATOR: char = ' ';

pub struct FENParser {}

impl FENParser {
    pub fn validate(fen_string: &'static str) -> bool {
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
    use super::FENParser;
    use super::game::Game;

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert!(FENParser::validate(initial_configuration));
    }
}