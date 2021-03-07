use std::str;
use serde_xml_rs::from_str;
use decimal::prelude::*;

use crate::meta_model::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_parse_works() {

    }
}
