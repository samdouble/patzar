use crate::engine::fenparser::errors::Error;
use crate::engine::game::Board;
use crate::engine::game::Color;
use crate::engine::game::Piece;
use crate::engine::game::Square;
use crate::engine::game::pieces::Bishop;
use crate::engine::game::pieces::King;
use crate::engine::game::pieces::Knight;
use crate::engine::game::pieces::Pawn;
use crate::engine::game::pieces::Queen;
use crate::engine::game::pieces::Rook;

const ROWS_SEPARATOR: char = '/';
const NUMBER_OF_ROWS: usize = 8;
const NUMBER_OF_COLS: usize = 8;

pub struct BoardBuilder {
    board: Board,
    next_to_play: Color,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            next_to_play: Color::White,
        }
    }

    pub fn set_configuration(&mut self, setting: &str) -> Result<&mut Self, Error> {
        let mut row_number = 0;
        for row in setting.split(ROWS_SEPARATOR) {
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
                            _ => Err(Error::InvalidPiece),
                        };
                        match piece {
                            Ok(piece) => {
                                self.board.add_piece(piece);
                                col_number += 1;
                            },
                            Err(_err) => return Err(Error::InvalidPiece),
                        }
                    },
                }
            };
            if col_number != NUMBER_OF_COLS {
                return Err(Error::WrongNumberOfCols);
            }
            // TODO more conditions ...
            row_number += 1;
        };
        if row_number != NUMBER_OF_ROWS {
            return Err(Error::WrongNumberOfRows);
        }
        Ok(self)
    }

    pub fn set_next_color_to_play(&mut self, setting: &str) -> Result<&mut Self, Error> {
        let color = match setting {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(Error::InvalidNextPlayerSetting),
        };
        self.next_to_play = color;
        Ok(self)
    }

    pub fn build(&self) -> Board {
        self.board.clone()
    }
}