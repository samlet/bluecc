#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests;
pub mod params;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseJson(serde_json::Error);
        ParseDateTimeErr(chrono::ParseError);
        ParseFloatErr(std::num::ParseFloatError);
    }
}

pub mod prelude {
    use std::borrow::Cow;
    use chrono::prelude::*;

    pub use super::params::{Object};

    const STD_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

    /// Call like: let s="2008-04-23 16:49:27.392";
    pub fn from_std_fmt(s:&str) -> crate::Result<DateTime<Utc>> {
        let dt:DateTime<Utc>=Utc.datetime_from_str(s, STD_FORMAT)?;
        Ok(dt)
    }

    pub fn utc_fmt(s:&str) -> crate::Result<String> {
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
