#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn eq_black() {
        assert!(Color::Black == Color::Black)
    }

    #[test]
    fn eq_white() {
        assert!(Color::White == Color::White)
    }

    #[test]
    fn neq_black_white() {
        assert!(Color::Black != Color::White)
    }
}
