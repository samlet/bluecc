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
    #[error("not found error")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

impl From<roxmltree::Error> for TestError {
    fn from(err: roxmltree::Error) -> TestError {
        TestError::ParseXml(err)
    }
}

fn find_by_attr<'input>(doc:&mut roxmltree::Document<'input>,
                    attr_name: &str, attr_val:&str) -> Result<String, TestError> {
    let found = doc.descendants().find(|n|
        n.attribute(attr_name) == Some(attr_val));
    match found {
        Some(n) => Ok(n.tag_name().name().to_string()),
        _ => Err(TestError::NotFound)
    }
}

#[test]
fn custom_err_works() -> anyhow::Result<()> {
    let xml_str=str::from_utf8(include_bytes!("ExampleDemoData.xml"))?;
    let mut doc = roxmltree::Document::parse(xml_str).unwrap();
    let node_name= find_by_attr(&mut doc, "statusTypeId", "EXAMPLE_STATUS")?;
    assert_eq!("StatusType", node_name);
    Ok(())
}

#[test]
fn get_entities_test() -> anyhow::Result<()> {
    let xml_str=str::from_utf8(include_bytes!("ExampleDemoData.xml"))?;
    let mut doc = roxmltree::Document::parse(xml_str).unwrap();
    let root = doc.root_element();
    // for node in root.children(){
    //     println!("{}", node.tag_name().name());
    // }
    let excepts=vec!["create", "create-replace", "create-update", "delete"];
    let ents=root.children().into_iter()
        .filter(|e| e.is_element() && !excepts.contains(&e.tag_name().name()))
        .map(|e| e.tag_name().name().to_string())
        .collect::<HashSet<String>>();
    let r=serde_json::to_string_pretty(&ents)?;
    println!("{}", r);
    Ok(())
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
    let skip_nodes=vec!["create", "create-replace", "create-update", "delete"];
    for n in iter{
        if !n.is_element() || skip_nodes.contains(&n.tag_name().name()){
            continue;
        }
        println!("{}", n.tag_name().name());
        for attr in n.attributes(){
            println!("\t{} = {}", attr.name(), attr.value());
        }
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
use std::collections::{HashMap, HashSet};
use structmap::value::Value;
use crate::util::parse_pair;

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

#[test]
fn parse_works() {
    use decimal::prelude::*;

    let x=parse_pair::<i32>("18");
    assert_eq!(18, x);
    let f=parse_pair::<f32>("18.01");
    assert_eq!(18.01, f);
    let d=parse_pair::<Decimal>("18.01");
    assert_eq!(Decimal::new(1801,2), d);

}

