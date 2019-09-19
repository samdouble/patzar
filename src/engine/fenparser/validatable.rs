pub trait Validatable<T, E> {
    fn validate(fen_string: &str) -> Result<T, E>;
}
