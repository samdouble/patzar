use super::super::Square::*;

const NO_ENPASSANT: char = '-';

pub struct FENSetting4 {}

// Fourth setting: en passant
impl FENSetting4 {
    pub fn validate(setting: &'static str) -> bool {
        if setting == NO_ENPASSANT.to_string() {
            return true;
        }
        if !Square::validate(setting) {
            return false;
        }
        // Because "en-passant" can only be realized on a pawn that has
        // moved 2 squares from its starting position (rows 2 and 7), it is possible
        // to be a little more specific than just validating that it is a valid square
        // string by checking that the row number is either 3 or 6.
        let square: Square = Square::new(setting);
        let square_row: u8 = square.get_row();
        square_row == 3 || square_row == 6
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting4;

    #[test]
    fn validate_dash() {
        let valid = FENSetting4::validate("-");
        assert!(valid);
    }

    #[test]
    fn validate_third_row() {
        let valid = FENSetting4::validate("c3");
        assert!(valid);
    }

    #[test]
    fn validate_sixth_row() {
        let valid = FENSetting4::validate("h6");
        assert!(valid);
    }

    #[test]
    fn invalidate_another_row_than_third_or_sixth() {
        let valid = FENSetting4::validate("h4");
        assert!(!valid);
    }
}