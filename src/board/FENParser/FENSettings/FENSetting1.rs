const NUMBER_OF_ROWS: u8 = 8;
const LINES_SEPARATOR: char = '/';

pub struct FENSetting1 {}

// First setting: position of every piece on the board
impl FENSetting1 {
    pub fn validate(setting: &'static str) -> bool {
        let lines = setting.split(LINES_SEPARATOR);

        let mut lines_count = 0;
        for (i, line) in lines.enumerate() {
            let squares: Vec<char> = line.chars().collect();
            for square in squares {
                println!("{}", square);
            }
            lines_count += 1;
        };
        if lines_count != NUMBER_OF_ROWS {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::FENSetting1;

    #[test]
    fn validate_initial_board() {
        let valid = FENSetting1::validate("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }

    #[test]
    fn invalidate_empty() {
        let valid = FENSetting1::validate("");
        assert!(!valid);
    }
}