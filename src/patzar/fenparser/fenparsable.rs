pub trait FENParsable<T> {
    fn from_FEN_string(fen_string: &str) -> Result<T, ()>;
}
