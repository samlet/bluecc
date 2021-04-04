#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate error_chain;

#[cfg(test)]
mod tests;
pub mod params;

// error_chain!{
//     types {
//         Error, ErrorKind, ResultExt, Result;
//     }
//     links {}
//     foreign_links {
//         Io(std::io::Error);
//         ParseInt(::std::num::ParseIntError);
//         ParseJson(serde_json::Error);
//         ParseDateTimeErr(chrono::ParseError);
//         ParseFloatErr(std::num::ParseFloatError);
//     }
// }

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CommonError {
        #[error("io error")]
        Io(#[from] std::io::Error),
        #[error("parse error")]
        Parse(#[from] std::num::ParseIntError),
        #[error("parse float error")]
        ParseFloatErr(#[from] std::num::ParseFloatError),
        #[error("json parse fail")]
        JsonSerialize(#[from] serde_json::Error),
        // #[error("decimal parse fail")]
        // ParseBigDecimalErr(#[from] bigdecimal::ParseBigDecimalError),
        #[error("date-time parse fail")]
        ParseDateTimeErr(#[from] chrono::ParseError),
    }
}

pub use error::CommonError as Error;

pub mod prelude {
    use std::borrow::Cow;
    use chrono::prelude::*;

    pub use super::params::{Object};
    pub type Result<T> = std::result::Result<T, super::error::CommonError>;

    const STD_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

    /// Call like: let s="2008-04-23 16:49:27.392";
    pub fn from_std_fmt(s:&str) -> Result<DateTime<Utc>> {
        let dt:DateTime<Utc>=Utc.datetime_from_str(s, STD_FORMAT)?;
        Ok(dt)
    }

    pub fn utc_fmt(s:&str) -> Result<String> {
        let dt:DateTime<Utc>=Utc.datetime_from_str(s, STD_FORMAT)?;
        Ok(dt.to_rfc3339_opts(SecondsFormat::Millis, false).to_string())
    }

    pub fn to_std_fmt(dt:&DateTime<Utc>) -> String {
        dt.format(STD_FORMAT).to_string()
    }

    pub fn to_utc_fmt(dt:&DateTime<Utc>) -> String {
        dt.to_rfc3339_opts(SecondsFormat::Millis, false).to_string()
    }

    pub fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
        if input.contains(' ') {
            let mut buf = String::with_capacity(input.len());

            for c in input.chars() {
                if c != ' ' {
                    buf.push(c);
                }
            }

            return Cow::Owned(buf);
        }

        return Cow::Borrowed(input);
    }
}

#[cfg(test)]
mod common_tests {
    use crate::prelude::*;

    #[test]
    fn remove_spaces_works() {
        let input_str="assert_eq!(2 + 2, 4);";
        let result=remove_spaces(input_str);
        println!("{}", result.to_string());
    }
}
