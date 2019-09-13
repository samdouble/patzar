pub trait FENParsable<T, E> {
    fn from_FEN_string(fen_string: &str) -> Result<T, E>;
}
