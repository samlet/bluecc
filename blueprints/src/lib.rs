#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
#[macro_use]
extern crate maplit;

mod store;
mod graph_meta;
mod rules;

mod error{
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum BluesError {
        #[error("io error")]
        Io(#[from] std::io::Error),
        #[error("parse error")]
        Parse(#[from] std::num::ParseIntError),
        #[error("time error")]
        SystemTimeErr(#[from] std::time::SystemTimeError),
        #[error("json parse fail")]
        JsonSerialize(#[from] serde_json::Error),
        #[error("request fail")]
        RequestErr(#[from] reqwest::Error),
        // #[error("generic error")]
        // GenericErr(#[from] seed::GenericError),
        // #[error("service error")]
        // ServiceErr(#[from] deles::ServiceError),
    }
}

pub type Result<T> = std::result::Result<T, error::BluesError>;

#[cfg(test)]
mod tests {
    #[test]
    fn map_works() {
        #[derive(Debug, Clone, PartialEq)]
        struct User {
            username: String,
        }
        // assert_eq!(2 + 2, 4);
        let map= hashmap! { "John".to_string() =>
            User { username: "John".to_string() } };
        println!("{:?}", map);
    }
}

