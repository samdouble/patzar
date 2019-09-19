use crate::engine::fenparser::errors::Error;
use crate::engine::fenparser::fensettings::FENSetting1;
use crate::engine::fenparser::fensettings::FENSetting2;
use crate::engine::fenparser::fensettings::FENSetting3;
use crate::engine::fenparser::fensettings::FENSetting4;
use crate::engine::fenparser::fensettings::FENSetting5;
use crate::engine::fenparser::fensettings::FENSetting6;
use crate::engine::fenparser::Validatable;
use crate::engine::game::Board;
use crate::engine::game::CastlingOption;
use crate::engine::game::Color;
use crate::engine::game::Square;

pub struct BoardBuilder {
    board: Board,
    next_to_play: Color,
    castling_options: Option<Vec<CastlingOption>>,
    en_passant: Option<Square>,
    nb_half_moves: u8,
    nb_full_moves: u8,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            next_to_play: Color::White,
            castling_options: Some(vec![
                CastlingOption::KingSide(Color::White),
                CastlingOption::QueenSide(Color::White),
                CastlingOption::KingSide(Color::Black),
                CastlingOption::QueenSide(Color::Black),
            ]),
            en_passant: None,
            nb_half_moves: 0,
            nb_full_moves: 0,
        }
    }

    pub fn set_configuration(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting1::validate(setting) {
            Ok(pieces) => {
                self.board.add_piece(pieces[0]);
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_next_color_to_play(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting2::validate(setting) {
            Ok(color) => {
                self.next_to_play = color;
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_castling_options(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting3::validate(setting) {
            Ok(castling_options) => {
                self.castling_options = castling_options;
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_en_passant_target(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting4::validate(setting) {
            Ok(en_passant) => {
                self.en_passant = en_passant;
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_half_moves_counter(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting5::validate(setting) {
            Ok(nb_half_moves) => {
                self.nb_half_moves = nb_half_moves;
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn set_nb_full_moves(&mut self, setting: &str) -> Result<&mut Self, Error> {
        match FENSetting6::validate(setting) {
            Ok(nb_full_moves) => {
                self.nb_full_moves = nb_full_moves;
                Ok(self)
            },
            Err(err) => Err(err),
        }
    }

    pub fn build(&self) -> Board {
        self.board.clone()
    }
}