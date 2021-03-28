#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

pub mod runner;
mod script_module;
mod errors;
mod kube;
mod srv_wrapper;

pub use self::errors::GenericError;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    links {}
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
        ParseJson(serde_json::Error);
        ConfigTomlErr(toml::de::Error);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
