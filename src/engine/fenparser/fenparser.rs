use crate::engine::fenparser::fensettings::FENSetting1;
use crate::engine::fenparser::fensettings::FENSetting2;
use crate::engine::fenparser::fensettings::FENSetting3;
use crate::engine::fenparser::fensettings::FENSetting4;
use crate::engine::fenparser::fensettings::FENSetting5;
use crate::engine::fenparser::fensettings::FENSetting6;
use crate::engine::fenparser::Validatable;

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
    use crate::engine::fenparser::FENParser;
    use crate::engine::fenparser::Validatable;
    use crate::engine::game::Board;

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Board::get_initial_configuration();
        assert!(FENParser::validate(initial_configuration));
    }

    // TODO more tests
}
