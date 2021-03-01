#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

// #[cfg(test)]
pub mod tests;
mod cases;
mod blues;

#[cfg(test)]
mod lib_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



