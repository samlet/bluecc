use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

// ref: https://github.com/dtolnay/serde-yaml
#[test]
fn yaml_works() {
    let mut map = BTreeMap::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);

    // Serialize it to a YAML string.
    let s = serde_yaml::to_string(&map).expect("serialize to yaml");
    assert_eq!(s, "---\nx: 1.0\ny: 2.0\n");

    // Deserialize it back to a Rust type.
    let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&s).expect("back to map");
    assert_eq!(map, deserialized_map);
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn test_struct() -> Result<(), serde_yaml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_yaml::to_string(&point)?;
    assert_eq!(s, "---\nx: 1.0\ny: 2.0\n");

    let deserialized_point: Point = serde_yaml::from_str(&s)?;
    assert_eq!(point, deserialized_point);
    Ok(())
}

#[test]
fn derive_struct_works() {
    test_struct().expect("struct works");
}
