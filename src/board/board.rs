pub struct Board {}

impl Board {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    /*fn validate(fen_string: &'static str) -> bool {
        true
    }*/
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn get_initial_configuration() {
        assert_eq!(Board::get_initial_configuration(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}