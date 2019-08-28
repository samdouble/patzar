use super::FENParser::FENParser;

pub struct Board {}

impl Board {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    pub fn validate(fen_string: &'static str) -> bool {
        FENParser::validate(fen_string)
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