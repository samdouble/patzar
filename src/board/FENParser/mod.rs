use std::collections::HashMap;
mod fensetting1;
use fensetting1::FENSetting1;
mod fensetting2;
use fensetting2::FENSetting2;

pub struct FENParser {}

impl FENParser {
    pub fn validate(fen_string: &'static str) -> bool {
        let mut settings = fen_string.split(" ");

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
            if (!setting_valid) {
                return false;
            }
        };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::FENParser;

    /*#[test]
    fn validate() {
        assert_eq!(FENParser::validate("eav fbe"), false);
    }*/
}