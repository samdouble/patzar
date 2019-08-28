pub struct FENOption1 {}

impl FENOption1 {
    pub fn validate(option: &'static str) -> bool {
        option == 'w' || option == 'b';
    }
}

#[cfg(test)]
mod tests {
    use super::FENOption1;

    /*#[test]
    fn get_initial_configuration() {
        assert_eq!(true, true);
    }*/
}