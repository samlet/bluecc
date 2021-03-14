#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use] extern crate log;
pub mod runner;
mod script_module;
mod errors;
mod kube;

pub use self::errors::GenericError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
