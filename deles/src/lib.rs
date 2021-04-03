#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;

pub mod delegators;
mod resources;
mod ds;

pub use seed::{GenericError};

// error_chain!{
//     types {
//         Error, ErrorKind, ResultExt, Result;
//     }
//     links {}
//     foreign_links {
//         Io(std::io::Error);
//         ParseInt(::std::num::ParseIntError);
//         ParseJson(serde_json::Error);
//         GenericErr(seed::GenericError);
//         RequestErr(reqwest::Error);
//         ParseBigDecimalErr(bigdecimal::ParseBigDecimalError);
//         ParseDateTimeErr(chrono::ParseError);
//         ParseFloatErr(std::num::ParseFloatError);
//         ParseXmlErr(serde_xml_rs::Error);
//         DatabaseErr(quaint::error::Error);
//         CommonErr(common::Error);
//     }
//
//     errors {
//         DataFormatError(t: String){
//             description("invalid data format")
//             display("invalid data format: '{}'", t)
//         }
//     }
// }

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ServiceError {
        #[error("io error")]
        Io(#[from] std::io::Error),
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
        #[error("database error")]
        DatabaseErr(#[from] quaint::error::Error),
        #[error("deles error")]
        GenericErr(#[from] seed::GenericError),
        #[error("config error")]
        ConfigErr,
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader {
            expected: String,
            found: String,
        },
        #[error("Error reading script file: {file_name:?}; {info:?}")]
        ScriptError {
            file_name: String,
            info: String,
        },
        #[error("Error finding: {item_name:?}; {info:?}")]
        NotFound {
            item_name: String,
            info: String,
        },
        #[error("Data format error: {info:?}")]
        DataFormatError {
            info: String,
        },
        #[error("unknown error")]
        Unknown,
        #[error(transparent)]
        Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
    }

    impl warp::reject::Reject for ServiceError {}
}

pub type Result<T> = std::result::Result<T, error::ServiceError>;
pub use error::ServiceError;

#[cfg(test)]
mod tests {
    // use crate::delegators::print_errs;

    /*
    fn foo() -> crate::Result<()> {
        if true==false {
            bail!(crate::ErrorKind::DataFormatError("xxx".to_string()));
        } else {
            Ok(())
        }
    }

    /// https://docs.rs/error-chain/0.12.4/error_chain/
    #[test]
    fn err_works() {
        assert_eq!(2 + 2, 4);
        let r:crate::Result<()>=Err(crate::ErrorKind::DataFormatError(
            format!("occurs a err at {}", chrono::Utc::now())).into());
        // println!("{:?}", r);
        if let Err(ref errors) = r {
            print_errs(errors);
        }
    }
     */
}
