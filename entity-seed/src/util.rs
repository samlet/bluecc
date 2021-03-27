use serde::Deserialize;
use std::io::{Read, BufReader};

/// Attempts to convert the given &str into a T, panicing if it's not successful
pub fn parse_pair<T>(v: &str) -> T where T : ::std::str::FromStr {
    let res = v.parse::<T>();
    match res {
        Ok(val) => val,
        Err(_) => panic!("Unable to convert given input into required type: {}", v),
    }
}

pub fn deserialize_branch_without_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader));
    T::deserialize(&mut de).unwrap()
}

/// https://gist.github.com/tobz1000/dd2d91c1e8c63171a21ec2d51dc726c7
pub fn deserialize_branch_with_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader))
        .non_contiguous_seq_elements(true);
    T::deserialize(&mut de).unwrap()
}

