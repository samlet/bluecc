#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate bson;

#[cfg(test)]
mod tests;
pub mod api;

use std::borrow::Cow;

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

#[cfg(test)]
mod common_tests {
    use crate::remove_spaces;

    #[test]
    fn remove_spaces_works() {
        let input_str="assert_eq!(2 + 2, 4);";
        let result=remove_spaces(input_str);
        println!("{}", result.to_string());
    }
}
