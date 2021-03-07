// use serde_xml_rs::{from_reader, from_str};
use std::str;
use std::io::prelude::*;
use crate::meta_model::EntityModel;
use serde::Deserialize;
use std::io::{Read, BufReader};
use std::collections::{HashMap, BTreeMap};
use chrono::Utc;
use serde_json::{Value, Error};
use crate::util::parse_pair;

lazy_static_include_bytes! {
// lazy_static_include_str! {
    EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
    ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
}

lazy_static! {
    pub static ref ENTITY_FACTORY: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m
    };
}

fn deserialize_branch_without_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader));
    T::deserialize(&mut de).unwrap()
}

/// https://gist.github.com/tobz1000/dd2d91c1e8c63171a21ec2d51dc726c7
fn deserialize_branch_with_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader))
        .non_contiguous_seq_elements(true);
    T::deserialize(&mut de).unwrap()
}


#[test]
fn doc_works() -> anyhow::Result<()>{
    // let _ = simple_logger::init();
    // let model:EntityModel=from_str(str::from_utf8(&EXAMPLE_DOC).unwrap())?;
    // let model:EntityModel=from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap())?;
    let model:EntityModel=deserialize_branch_with_contiguous_check(&**ACCOUNTING_DOC);
    // let model:EntityModel= match from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap()) {
    //     Ok(doc) => doc,
    //     Err(e) => {
    //         eprintln!("Error reading: {:#?}", e);
    //         panic!()
    //     }
    // };
    println!("{:?}, contains {} entities, {} views", model.title,
             model.entities.len(), model.views.len());
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExampleStatusSeed{
    // keys
    #[serde(rename = "exampleId", default)]
    pub example_id: String,
    #[serde(rename = "statusDate")]
    pub status_date: chrono::NaiveDateTime,
    // fields
    #[serde(rename = "statusEndDate")]
    pub status_end_date: chrono::NaiveDateTime,
    #[serde(rename = "changeByUserLoginId", default)]
    pub change_by_user_login_id: i64,
    #[serde(rename = "statusId", default)]
    pub status_id: String
}


#[test]
fn de_works() {
    let _ = simple_logger::init();
    let xml_str=r##"<ExampleStatusSeed exampleId="EX01" statusDate="2007-04-05T14:30:30"
    statusEndDate="2007-04-05T14:30:30" statusId="EXST_IN_DESIGN"/>"##;
    let data:ExampleStatusSeed=serde_xml_rs::from_str(xml_str).unwrap();
    println!("{:?}", data);
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExampleStatus{
    // keys
    #[serde(rename = "exampleId", default)]
    pub example_id: i32,
    #[serde(rename = "statusDate")]
    pub status_date: chrono::NaiveDateTime,
    // fields
    #[serde(rename = "statusEndDate")]
    pub status_end_date: chrono::NaiveDateTime,
    #[serde(rename = "changeByUserLoginId", default)]
    pub change_by_user_login_id: i64,
    #[serde(rename = "statusId", default)]
    pub status_id: i32
}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SerialKey{
    pub entity_name: String,
    pub field_name: String,
    pub field_value: String
}

struct StringStore{
    serial_id: i32,
    pub string_store: BTreeMap<SerialKey, i32>
}

impl StringStore{
    pub fn new() -> Self {
        StringStore { serial_id: (1), string_store:BTreeMap::new() }
    }
    pub fn id(&mut self, key: SerialKey) -> i32{
        let val= self.string_store.entry(key).or_insert(self.serial_id);
        self.serial_id+=1;
        *val
    }
}

/// Strips ns precision from `Utc::now`. PostgreSQL only has microsecond
    /// precision, but some platforms (notably Linux) provide nanosecond
    /// precision, meaning that round tripping through the database would
    /// change the value.
fn now() -> chrono::NaiveDateTime {
    let now = Utc::now().naive_utc();
    let nanos = now.timestamp_subsec_nanos();
    now - chrono::Duration::nanoseconds(nanos.into())
}

#[test]
fn seed_works() -> anyhow::Result<()>{
    use chrono::{NaiveDateTime, NaiveDate};
    let parse_from_str = NaiveDateTime::parse_from_str;

    let mut string_store=StringStore::new();

    let seed=ExampleStatus{
        example_id: string_store.id(SerialKey{
            entity_name: "ExampleStatus".to_string(),
            field_name: "exampleId".to_string(),
            field_value: "EX01".to_string()
        }),
        status_date: now(),
        status_end_date: parse_from_str("2022-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?,
        change_by_user_login_id: 0,
        status_id: string_store.id(SerialKey{
            entity_name: "ExampleStatus".to_string(),
            field_name: "statusId".to_string(),
            field_value: "EXST_IN_DESIGN".to_string()
        })
    };
    println!("{:?}", seed);
    for (k,v) in &string_store.string_store{
        if *v==2{
            println!("{:?}", k);
        }
    }
    assert_eq!(2, string_store.id(SerialKey{
            entity_name: "ExampleStatus".to_string(),
            field_name: "statusId".to_string(),
            field_value: "EXST_IN_DESIGN".to_string()
        }));

    Ok(())
}

// use decimal::prelude::*;
use serde::de::DeserializeOwned;

#[derive(Deserialize, Debug)]
struct User {
    pub fingerprint: String,
    pub location: String,
    pub money: f32,
    pub age: i32,
}

fn create_entity<T>(map:&BTreeMap<&str, Value>) -> Result<T, Error>
    where T:DeserializeOwned {
    let val:Value=serde_json::to_value(map).unwrap();
    serde_json::from_value::<T>(val)
}

#[test]
fn as_string_map_works() {
    let mut map = BTreeMap::new();
    map.insert("fingerprint", Value::from("XXXXX"));
    map.insert("location", Value::from("Menlo Park, CA"));
    map.insert("age", Value::from(parse_pair::<i32>("18")));
    map.insert("money", Value::from(parse_pair::<f32>("18.01")));
    let val=create_entity::<User>(&map);
    println!("{:?}", val.unwrap());
    let f = create_entity::<User>;
    println!("{:?}", f(&map).unwrap())
}

