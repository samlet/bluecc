#[derive(Debug)]
enum Thing {
    One(i32),
    Two(String),
    Unknown,
}

impl Thing {
    fn is_unknown(&self) -> bool {
        match *self {
            Thing::Unknown => true,
            _ => false,
        }
    }
}

#[test]
fn enum_filter() {
    let things = vec![Thing::One(42), Thing::Two("hello".into()), Thing::Unknown];
    for t in things.iter().filter(|s| !s.is_unknown()) {
        println!("{:?}", t);
    }
}

#[test]
fn if_let_works() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(0u8) = some_u8_value {
        println!("three");
    }
}

#[test]
fn enum_struct_works() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);
}