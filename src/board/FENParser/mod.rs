use std::collections::HashMap;
mod fenoption1;
use fenoption1::FENOption1;

pub struct FENParser {}

lazy_static!{
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

impl FENParser {
    pub fn validate(fen_string: &'static str) -> bool {
        let mut options = fen_string.split(" ");

        for (i, option) in options.enumerate() {
            let setting_valid = FENOption1::validate(option);
            println!("{} {}", i, setting_valid);
        };
        for (k, v) in HASHMAP.iter() {
            println!("{:?}: {:?}", k, v);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::FENParser;

    /*#[test]
    fn validate() {
        assert_eq!(FENParser::validate("eav fbe"), false);
    }*/
}