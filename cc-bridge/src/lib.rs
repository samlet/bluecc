#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

pub mod proposals;
pub mod srv_director;
pub mod eth;

pub mod error {
    use thiserror::Error;
    use std::str::Utf8Error;

    #[derive(Error, Debug)]
    pub enum BridgeError {
        #[error("io error")]
        Io(#[from] std::io::Error),
        #[error("utf8 error")]
        Utf8Err(#[from] Utf8Error),
        #[error("parse error")]
        Parse(#[from] std::num::ParseIntError),
        #[error("parse float error")]
        ParseFloatErr(#[from] std::num::ParseFloatError),
        #[error("json parse fail")]
        JsonSerialize(#[from] serde_json::Error),
        #[error("decimal parse fail")]
        ParseBigDecimalErr(#[from] bigdecimal::ParseBigDecimalError),
        #[error("date-time parse fail")]
        ParseDateTimeErr(#[from] chrono::ParseError),
        #[error("request fail")]
        RequestErr(#[from] reqwest::Error),
        #[error("rabbitmq fail")]
        RabbitErr(#[from] lapin::Error),
        #[error("web3 fail")]
        Web3Err(#[from] web3::Error),
        #[error("ethabi fail")]
        AbiErr(#[from] web3::ethabi::Error),
        #[error("deles error")]
        GenericErr(#[from] seed::GenericError),
        #[error("config error")]
        ConfigErr,
        #[error("common error")]
        CommonErr(#[from] common::Error),
        #[error("Error finding: {item_name:?}; {info:?}")]
        NotFound {
            item_name: String,
            info: String,
        }
    }
}

pub type Result<T> = std::result::Result<T, error::BridgeError>;
pub use error::BridgeError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 2, 3);
    }
}
