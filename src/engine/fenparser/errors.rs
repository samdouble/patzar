#[derive(Debug)]
pub enum Error {
    // Setting1
    InvalidPiece,
    // Setting2
    InvalidNextPlayerSetting,
    InvalidSquareRepresentation,
    // Setting3
    InvalidCastlingOptionSetting,
    CastlingOptionSpecifiedTwice,
    // Setting4
    InvalidEnPassantSetting,
    InvalidEnPassantTargetSquare,
    // Setting5
    InvalidHalfMovesSetting,
    // Setting6
    InvalidFullMovesSetting,
    //MoreThanOneKing,
    WrongNumberOfCols,
    WrongNumberOfRows,
}
