use regex::Regex;
use crate::patzar::fenparser::validatable::Validatable;

pub struct Square {
    row: u8,
    col: char,
}

impl Square {
    pub fn new(fen_position: &str) -> Self {
        if !Self::validate(fen_position) {
            panic!("Invalid square position: {}", fen_position);
        }
        Self {
            row: fen_position
                .chars().nth(1).unwrap().to_string()
                .parse::<u8>().unwrap(),
            col: fen_position.chars().nth(0).unwrap(),
        }
    }

    pub fn get_row(&self) -> u8 {
        self.row
    }
}

impl Validatable for Square {
    fn validate(setting: &str) -> bool {
        let square_regex: Regex = Regex::new(r"^[a-h][1-8]$").unwrap();
        square_regex.is_match(setting)
    }
}

#[cfg(test)]
mod tests {
    use super::Square;
    use crate::patzar::fenparser::validatable::Validatable;

    #[test]
    fn validate_a1() {
        let valid = Square::validate("a1");
        assert!(valid);
    }

    #[test]
    fn validate_a8() {
        let valid = Square::validate("a8");
        assert!(valid);
    }

    #[test]
    fn validate_h1() {
        let valid = Square::validate("h1");
        assert!(valid);
    }

    #[test]
    fn validate_h8() {
        let valid = Square::validate("h8");
        assert!(valid);
    }

    #[test]
    fn validate_e5() {
        let valid = Square::validate("e5");
        assert!(valid);
    }

    #[test]
    fn invalidate_wrong_order_colrow() {
        let valid = Square::validate("5e");
        assert!(!valid);
    }

    #[test]
    fn invalidate_capital_letter_for_col() {
        let valid = Square::validate("E5");
        assert!(!valid);
    }

    #[test]
    fn invalidate_out_of_bounds_col() {
        let valid = Square::validate("i1");
        assert!(!valid);
    }

    #[test]
    fn invalidate_out_of_bounds_row() {
        let valid = Square::validate("a9");
        assert!(!valid);
    }
}