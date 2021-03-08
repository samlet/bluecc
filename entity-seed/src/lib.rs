#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lazy_static_include;
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod tests;
mod cases;
mod blues;
pub mod meta_model;
pub mod util;
pub mod schema;

#[cfg(test)]
mod lib_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



