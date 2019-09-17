#[derive(Debug)]
pub enum Error {
    // Setting1
    InvalidPiece,
    //MoreThanOneKing,
    WrongNumberOfCols,
    WrongNumberOfRows,
    // Setting2
    InvalidNextPlayerSetting,

    InvalidSquareRepresentation,
}
