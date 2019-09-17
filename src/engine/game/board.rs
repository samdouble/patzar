use crate::engine::fenparser::BoardBuilder;
use crate::engine::fenparser::errors::Error;
use crate::engine::fenparser::FENParsable;
use crate::engine::game::Piece;
use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;

const NUMBER_OF_SETTINGS: u8 = 6;
const SETTINGS_SEPARATOR: char = ' ';

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Vec::new(),
        }
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.pieces.push(piece);
    }

    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }

    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn get_possible_moves(fen_string: &str) -> Vec<Move> {
        let board = Board::from_FEN_string(fen_string)
            .expect("TODO invalid board configuration");

        let mut moves = Vec::new();
        for piece in board.get_pieces() {
            moves.append(&mut { piece.get_possible_moves() });
        }
        moves
    }
}

impl FENParsable<Self, Error> for Board {
    fn from_FEN_string(fen_string: &str) -> Result<Self, Error> {
        let mut settings = fen_string.split(SETTINGS_SEPARATOR);
        let setting1 = settings.nth(0).expect("TODO");
        let setting2 = settings.nth(1).expect("TODO");
        let setting3 = settings.nth(2).expect("TODO");
        let setting4 = settings.nth(3).expect("TODO");
        let setting5 = settings.nth(4).expect("TODO");
        let setting6 = settings.nth(5).expect("TODO");
        let mut board = BoardBuilder::new()
            .set_configuration(setting1).unwrap()
            .set_next_color_to_play(setting2).unwrap()
            .build();
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::game::Board;

    #[test]
    fn get_initial_configuration() {
        let initial_configuration: &'static str = Board::get_initial_configuration();
        assert_eq!(initial_configuration, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn get_possible_moves_with_initial_configuration() {
        let moves = Board::get_possible_moves("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(moves, Vec::new());
    }

    /*
    
    #[test]
    fn initial_configuration_is_valid() {
        let valid = Board::from_FEN_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }

    // TODO more tests*/
}
