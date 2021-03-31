#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

mod semantics;

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
