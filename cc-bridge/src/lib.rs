#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

mod proposals;

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;

// pub use seed::{GenericError};

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseJson(serde_json::Error);
        // GenericErr(seed::GenericError);
        RequestErr(reqwest::Error);
        ParseBigDecimalErr(bigdecimal::ParseBigDecimalError);
        ParseDateTimeErr(chrono::ParseError);
        ParseFloatErr(std::num::ParseFloatError);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 2, 3);
    }
}
