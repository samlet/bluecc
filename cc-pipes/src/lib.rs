#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod runner;
mod script_module;
mod errors;
mod kube;
mod srv_wrapper;

pub use self::errors::GenericError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
