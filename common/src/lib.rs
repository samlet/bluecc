#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate bson;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod common_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
