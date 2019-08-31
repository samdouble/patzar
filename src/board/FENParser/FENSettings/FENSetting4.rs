use super::super::Square::*;

const NO_ENPASSANT: char = '-';

pub struct FENSetting4 {}

// Fourth setting: en passant
impl FENSetting4 {
    pub fn validate(setting: &'static str) -> bool {
        if setting == NO_ENPASSANT.to_string() {
            return true;
        }
        Square::validate(setting)
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting4;

    /*#[test]
    fn validate_no_castling_options() {
        let valid = FENSetting3::validate("-");
        assert!(valid);
    }*/
}