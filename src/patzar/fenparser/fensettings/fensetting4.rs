use crate::patzar::fenparser::fenparsable::FENParsable;
use crate::patzar::fenparser::validatable::Validatable;
use super::super::square::Square;

const NO_ENPASSANT: char = '-';

pub struct FENSetting4 {}

// Fourth setting: en passant
impl Validatable for FENSetting4 {
    fn validate(setting: &str) -> bool {
        if setting == NO_ENPASSANT.to_string() {
            return true;
        }
        let square = match Square::from_FEN_string(setting) {
            Ok(square) => square,
            Err(_err) => return false,
        };
        // Because "en-passant" can only be realized on a pawn that has
        // moved 2 squares from its starting position (rows 2 and 7), it is possible
        // to be a little more specific than just validating that it is a valid square
        // string by checking that the row number is either 3 or 6.
        let square_row: usize = square.get_row();
        square_row == 2 || square_row == 5
    }
}

#[cfg(test)]
mod tests {
    use crate::patzar::fenparser::validatable::Validatable;
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