use crate::patzar::fenparser::FENParsable;
use super::fenparser::Board;
use super::fenparser::Move;

pub struct Game {}

impl Game {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    pub fn get_available_moves(fen_string: &str) -> Vec<&str> {
        let board = match Board::from_FEN_string(fen_string) {
            Ok(board) => board,
            Err(err) => panic!(err),
        };
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn get_initial_configuration() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert_eq!(initial_configuration, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    // TODO tests get_available_moves
}
