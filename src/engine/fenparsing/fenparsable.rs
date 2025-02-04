pub trait FENParsable<T> {
    fn from_fen_string(fen_string: &str) -> Result<T, ()>;
}
