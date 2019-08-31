pub struct Game {}

impl Game {
    pub fn get_initial_configuration() -> &'static str {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn get_initial_configuration() {
        let initial_configuration: &'static str = Game::get_initial_configuration();
        assert_eq!(initial_configuration, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}