use serde::{Serialize, Deserialize};
use serde_json::{json, Value, Result};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn json_works() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[test]
fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

#[test]
fn as_map_works() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
    }
    let mut map = BTreeMap::new();
    map.insert("fingerprint", "XXXXX");
    map.insert("location", "Menlo Park, CA");
    let val:Value=serde_json::to_value(map).unwrap();
    println!("{}", val);
    println!("{:?}", serde_json::from_value::<User>(val).unwrap())
}

#[test]
fn as_string_map_works() {
    #[derive(Deserialize, Debug)]
    struct User {
        fingerprint: String,
        location: String,
        age: i32,
    }
    let mut map = BTreeMap::new();
    map.insert("fingerprint", Value::from("XXXXX"));
    map.insert("location", Value::from("Menlo Park, CA"));
    map.insert("age", Value::from(12));
    let val:Value=serde_json::to_value(map).unwrap();
    println!("{}", val);
    println!("{:?}", serde_json::from_value::<User>(val).unwrap())
}

