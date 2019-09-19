use crate::engine::fenparser::Validatable;
use crate::engine::fenparser::errors::Error;
use crate::engine::fenparser::FENParsable;
use crate::engine::game::Square;

const NO_ENPASSANT: char = '-';

pub struct FENSetting4 {}

// Fourth setting: en passant
impl Validatable<Option<Square>, Error> for FENSetting4 {
    fn validate(setting: &str) -> Result<Option<Square>, Error> {
        if setting == NO_ENPASSANT.to_string() {
            return Ok(None);
        }
        let square = match Square::from_FEN_string(setting) {
            Ok(square) => square,
            Err(_err) => return Err(Error::InvalidEnPassantSetting),
        };
        // Because "en-passant" can only be realized on a pawn that has
        // moved 2 squares from its starting position (rows 2 and 7), it is possible
        // to be a little more specific than just validating that it is a valid square
        // string by checking that the row number is either 3 or 6.
        let square_row: usize = square.get_row();
        if square_row == 2 || square_row == 5 {
            Ok(Some(square))
        } else {
            Err(Error::InvalidEnPassantTargetSquare)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::fensettings::FENSetting4;
    use crate::engine::fenparser::Validatable;

    #[test]
    fn validate_dash() {
        let valid = match FENSetting4::validate("-") {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_third_row() {
        let valid = match FENSetting4::validate("c3") {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn validate_sixth_row() {
        let valid = match FENSetting4::validate("h6") {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(valid);
    }

    #[test]
    fn invalidate_another_row_than_third_or_sixth() {
        let valid = match FENSetting4::validate("h4") {
            Ok(_square) => true,
            Err(_err) => false,
        };
        assert!(!valid);
    }
}