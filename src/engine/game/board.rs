use crate::engine::fenparser::FENParsable;
use crate::engine::game::Color;
use crate::engine::game::Piece;
use crate::engine::game::Square;
use crate::engine::game::moves::Movable;
use crate::engine::game::moves::Move;
use crate::engine::game::pieces::King;
use crate::engine::game::pieces::Queen;
use crate::engine::game::pieces::Rook;
use crate::engine::game::pieces::Bishop;
use crate::engine::game::pieces::Knight;
use crate::engine::game::pieces::Pawn;

const NUMBER_OF_ROWS: usize = 8;
const NUMBER_OF_COLS: usize = 8;
const ROWS_SEPARATOR: char = '/';

pub struct Board {
    pieces: Vec<Piece>,
}

impl Board {
    pub fn get_pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    pub fn get_possible_moves(self) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in self.get_pieces() {
            moves.append(&mut { piece.get_possible_moves() });
        }
        moves
    }
}

impl FENParsable<Self> for Board {
    fn from_FEN_string(fen_string: &str) -> Result<Self, ()> {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut row_number = 0;
        for row in fen_string.split(ROWS_SEPARATOR) {
            let squares: Vec<char> = row.chars().collect();
            let mut col_number = 0;
            for square in squares {
                match square.to_string().parse::<usize>() {
                    Ok(num_squares) => col_number += num_squares,
                    Err(_err) => {
                        let position = Square::new(row_number, col_number);
                        let piece = match &square {
                            'K' => Ok(Piece::King(King::new(Color::White, position))),
                            'k' => Ok(Piece::King(King::new(Color::Black, position))),
                            'Q' => Ok(Piece::Queen(Queen::new(Color::White, position))),
                            'q' => Ok(Piece::Queen(Queen::new(Color::Black, position))),
                            'R' => Ok(Piece::Rook(Rook::new(Color::White, position))),
                            'r' => Ok(Piece::Rook(Rook::new(Color::Black, position))),
                            'B' => Ok(Piece::Bishop(Bishop::new(Color::White, position))),
                            'b' => Ok(Piece::Bishop(Bishop::new(Color::Black, position))),
                            'N' => Ok(Piece::Knight(Knight::new(Color::White, position))),
                            'n' => Ok(Piece::Knight(Knight::new(Color::Black, position))),
                            'P' => Ok(Piece::Pawn(Pawn::new(Color::White, position))),
                            'p' => Ok(Piece::Pawn(Pawn::new(Color::Black, position))),
                            _ => Err(()),
                        };
                        match piece {
                            Ok(piece) => {
                                pieces.push(piece);
                                col_number += 1;
                            },
                            Err(_err) => return Err(()),
                        }
                    },
                }
            };
            if col_number != NUMBER_OF_COLS {
                return Err(());
            }
            // TODO more conditions ...
            row_number += 1;
        };
        if row_number != NUMBER_OF_ROWS {
            return Err(());
        }
        Ok(Self { pieces })
    }
}

#[cfg(test)]
mod tests {
    /*use crate::engine::game::Board;
    
    #[test]
    fn initial_configuration_is_valid() {
        let valid = Board::from_FEN_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert!(valid);
    }

    // TODO more tests*/
}
