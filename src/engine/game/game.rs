use crate::engine::fenparser::FENParsable;
use crate::engine::game::Board;
use crate::engine::game::moves::Move;

pub struct Game {}

impl Game {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    pub fn get_possible_moves(fen_string: &str) -> Vec<Move> {
        /*let board = match Board::from_FEN_string(fen_string) {
            Ok(board) => board,
            Err(err) => panic!(err),
        };*/
        let board = Board::from_FEN_string(fen_string)
            .expect("TODO invalid board configuration");
        let moves = board.get_possible_moves();
        moves
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::game::Game;

    #[test]
    fn get_initial_configuration() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert_eq!(initial_configuration, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    // TODO tests get_available_moves
}
