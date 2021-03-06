use std::str;
use roxmltree::Node;
use thiserror::Error;

#[derive(Error, Debug)]
enum TestError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Parse(std::num::ParseIntError),
    #[error("xml parse fail")]
    ParseXml(roxmltree::Error),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

impl From<roxmltree::Error> for TestError {
    fn from(err: roxmltree::Error) -> TestError {
        TestError::ParseXml(err)
    }
}

/// ref: https://github.com/RazrFalcon/roxmltree/blob/master/tests/dom-api.rs
#[test]
fn seed_works(){
    let xml_str=str::from_utf8(include_bytes!("ExampleDemoData.xml")).unwrap();
    let doc = roxmltree::Document::parse(xml_str).unwrap();
    let elem = doc.descendants().find(|n|
        n.attribute("statusTypeId") == Some("EXAMPLE_STATUS")).unwrap();
    assert!(elem.has_tag_name("StatusType"));

    let root_elem=doc.root().first_element_child();
    let iter=root_elem.unwrap().children();
    for n in iter{
        println!("{}", n.tag_name().name());
    }
}

#[test]
fn seed_file_works() -> Result<(), TestError>{
    use std::fs;
    let cnt=fs::read_to_string("data/example/ExampleDemoData.xml")?;
    let doc = roxmltree::Document::parse(cnt.as_str())?;
        // .map_err(TestError::ParseXml)?;

    doc.descendants().find(|n|
        n.attribute("statusTypeId") == Some("EXAMPLE_STATUS")).and_then(|n|{
        assert!(n.has_tag_name("StatusType"));
        Some(n)
    });

    Ok(())
}

#[test]
fn reader_works() {
    // https://simplabs.com/blog/2020/12/31/xml-and-rust/
    // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
    let xml_str=include_bytes!("simple.xml");
    let parser = xml::reader::EventReader::from_str(str::from_utf8(xml_str).unwrap());
    for event in parser {
        println!("{:?}", event.unwrap());
    }
}

use structmap::FromHashMap;
use structmap_derive::FromHashMap;
use std::collections::HashMap;
use structmap::value::Value;

#[test]
fn struct_map_works() {
    #[derive(FromHashMap)]
    struct TestStruct {
        name: String,
        value: String,
        age: i32,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            Self {
                name: String::new(),
                value: String::new(),
                age: 0,
            }
        }
    }

    // create a hashmap with key-value pairs
    let mut hm = HashMap::new();

    // `Value` is an enum wrapper to support genericized types, to support structs
    // with varying types for their fields.
    hm.insert(String::from("name"), Value::new(String::from("example")));
    hm.insert(String::from("value"), Value::new(String::from("some_value")));
    hm.insert(String::from("age"), Value::new(18));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_hashmap(hm);
    assert_eq!(test.name, "example");
    assert_eq!(test.value, "some_value");
    assert_eq!(test.age, 18);
}

/// Attempts to convert the given &str into a T, panicing if it's not successful
fn parse_pair<T>(v: &str) -> T where T : ::std::str::FromStr {
    let res = v.parse::<T>();
    match res {
        Ok(val) => val,
        Err(_) => panic!(format!("Unable to convert given input into required type: {}", v)),
    }
}

#[test]
fn parse_works() {
    let x:i32=parse_pair("18");
    assert_eq!(18, x);
    let f:f32=parse_pair("18.01");
    assert_eq!(18.01, f);
}

