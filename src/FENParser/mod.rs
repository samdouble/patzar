mod FENSettings;
use FENSettings::FENSetting1::*;
use FENSettings::FENSetting2::*;
use FENSettings::FENSetting3::*;
use FENSettings::FENSetting4::*;
use FENSettings::FENSetting5::*;
use FENSettings::FENSetting6::*;
pub mod Game;
pub mod Board;
pub mod Row;
pub mod Square;
pub mod Piece;

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
    use super::Game::*;

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert!(FENParser::validate(initial_configuration));
    }
}