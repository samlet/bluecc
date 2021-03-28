#[cfg(test)]
mod lib_tests {
    use super::*;

    use serde::Deserialize;
    use std::{collections::HashMap, fs::File};

    #[derive(Debug, Deserialize)]
    struct Config {
        boolean: bool,
        float: f32,
        map: HashMap<u8, char>,
        nested: Nested,
        tuple: (u32, u32),
        vec: Vec<Nested>,
    }

    #[derive(Debug, Deserialize)]
    struct Nested {
        a: String,
        b: char,
    }

    #[test]
    fn ron_works() {
        let input_path = format!("{}/test_files/test.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let config: Config = ron::de::from_reader(f).unwrap();
        println!("Config: {:?}", &config);
    }
}


