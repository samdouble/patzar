const NUMBER_OF_ROWS: u8 = 8;
const ROWS_SEPARATOR: char = '/';

pub struct Board {}

impl Board {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    pub fn validate(fen_board: &'static str) -> bool {
        let rows = fen_board.split(ROWS_SEPARATOR);

        let mut rows_count = 0;
        for (i, row) in rows.enumerate() {
            let squares: Vec<char> = row.chars().collect();
            for square in squares {
                println!("{}", square);
            }
            rows_count += 1;
        };
        if rows_count != NUMBER_OF_ROWS {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn get_initial_configuration() {
        let initial_configuration: &'static str = Board::get_initial_configuration();
        assert_eq!(initial_configuration, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Board::get_initial_configuration();
        assert!(Board::validate(initial_configuration));
    }
}