pub trait Validatable {
    fn validate(fen_string: &str) -> bool;
}
