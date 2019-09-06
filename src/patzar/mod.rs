use crate::patzar::fenparser::validatable::Validatable;
mod fenparser;
use fenparser::FENParser;
mod game;
use game::Game;

pub struct Patzar {}

impl Patzar {
    pub fn get_initial_configuration() -> &'static str {
        Game::get_initial_configuration()
    }

    pub fn validate(fen_string: &str) -> bool {
        FENParser::validate(fen_string)
    }
}

/*
TODO Tests

FEN/EPD support
GetInitialBoard() -> fenString
Validate(fenString) -> bool
GetAvailableMoves(fenString) -> list of Moves
GetAvailableMoves(fenString, Square) -> list of Moves
GetPossibleDestinations(fenString, Square) -> list of Squares
CalculateBoardAfterMove(fenString, Move) -> fenString
GetAlivePieces(fenString) -> list of Pieces
GetAlivePieces(fenString, Color) -> list of Pieces
IsKingInDanger(fenString, Color) -> bool
CalculateFavoriteForWin(fenString) -> Color, pct of winning
CalculateChancesOfWinning(fenString, Color) -> pct of winning
*/