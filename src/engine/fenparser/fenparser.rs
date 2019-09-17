use crate::engine::fenparser::FENParsable;
use crate::engine::fenparser::Validatable;
use crate::engine::game::Board;

pub struct FENParser {}

impl Validatable for FENParser {
    fn validate(fen_string: &str) -> bool {
        match Board::from_FEN_string(fen_string) {
            Ok(_board) => true,
            Err(_err) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::fenparser::FENParser;
    use crate::engine::fenparser::Validatable;
    use crate::engine::game::Board;

    #[test]
    fn initial_configuration_is_valid() {
        let initial_configuration: &'static str = Board::get_initial_configuration();
        assert!(FENParser::validate(initial_configuration));
    }

    // TODO more tests
}
