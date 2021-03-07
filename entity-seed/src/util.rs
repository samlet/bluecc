/// Attempts to convert the given &str into a T, panicing if it's not successful
pub fn parse_pair<T>(v: &str) -> T where T : ::std::str::FromStr {
    let res = v.parse::<T>();
    match res {
        Ok(val) => val,
        Err(_) => panic!(format!("Unable to convert given input into required type: {}", v)),
    }
}

