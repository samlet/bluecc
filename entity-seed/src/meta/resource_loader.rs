// use serde_xml_rs::{from_reader, from_str};
use std::str;
use std::io::prelude::*;
use crate::meta_model::EntityModel;
use serde::Deserialize;
use std::io::{Read, BufReader};
use std::collections::{HashMap, BTreeMap};
use chrono::Utc;
use serde_json::{Value, Error};
use crate::util::{parse_pair};
use std::fs;

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

#[test]
fn doc_works() -> anyhow::Result<()>{
    // let _ = simple_logger::init();
    // let model:EntityModel=from_str(str::from_utf8(&EXAMPLE_DOC).unwrap())?;
    // let model:EntityModel=from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap())?;
    let model:EntityModel=load_xml(&**ACCOUNTING_DOC);
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
    pub example_id: i64,
    #[serde(rename = "statusDate")]
    pub status_date: chrono::NaiveDateTime,
    // fields
    #[serde(rename = "statusEndDate")]
    pub status_end_date: chrono::NaiveDateTime,
    #[serde(rename = "changeByUserLoginId", default)]
    pub change_by_user_login_id: i64,
    #[serde(rename = "statusId", default)]
    pub status_id: i64
}

#[derive(Debug, Deserialize, Serialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SerialKey{
    pub entity_name: String,
    pub field_name: String,
    pub field_value: String
}

impl SerialKey {
    pub fn new(ent: &str, fld: &str, val: &str) -> Self{
        Self{
            entity_name: ent.to_string(),
            field_name: fld.to_string(),
            field_value: val.to_string()
        }
    }
}

pub struct StringStore{
    pub string_store: BTreeMap<SerialKey, i64>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StoreItem{
    pub key: SerialKey,
    pub id: i64
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct StoreItems{
    pub items: Vec<StoreItem>,
}

const STORE_FILE: &str =".store/id_store.json";
impl<'a> StringStore{
    pub fn new() -> Self {
        StringStore { string_store:BTreeMap::new() }
    }
    pub fn load() -> std::io::Result<Self>{
        use std::path::Path;
        if !Path::new(STORE_FILE).exists(){
            return Ok(Self::new())
        }

        let mut map=BTreeMap::new();
        let mut store:StoreItems=serde_json::from_reader(fs::File::open(STORE_FILE)?)?;
        for item in store.items{
            map.insert(item.key, item.id);
        }
        Ok(StringStore { string_store: map })
    }

    pub fn id(&mut self, key: SerialKey) -> i64{
        let serial_id=new_snowflake_id();
        let val= self.string_store.entry(key).or_insert(serial_id);
        *val
    }

    pub fn save(&self, file_name: &str) -> std::io::Result<()>{
        use std::fs::File;

        let mut serials=StoreItems{items:Vec::new()};
        for (entry,val) in &self.string_store {
            serials.items.push(StoreItem{ key: entry.clone(), id: *val });
        }
        let json_str=serde_json::to_string_pretty(&serials)?;
        let mut file = File::create(file_name)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    pub fn store(&self) -> std::io::Result<()>{
        fs::create_dir_all(".store")?;
        self.save(STORE_FILE)
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

use chrono::{NaiveDateTime, NaiveDate};
#[test]
fn seed_works() -> anyhow::Result<()>{
    let parse_dt=NaiveDateTime::parse_from_str;
    let mut string_store=StringStore::new();

    let seed=ExampleStatus{
        example_id: string_store.id(SerialKey::new(
             "ExampleStatus",
             "exampleId",
            "EX01"
        )),
        status_date: now(),
        status_end_date: parse_dt("2022-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?,
        change_by_user_login_id: 0,
        status_id: string_store.id(SerialKey::new(
            "ExampleStatus",
            "statusId",
            "EXST_IN_DESIGN"
        ))
    };
    println!("{:?}", seed);
    for (k,v) in &string_store.string_store{
            println!("{:?}, {}", k, v);
    }
    // assert_eq!(1, string_store.id(SerialKey{
    //         entity_name: "ExampleStatus",
    //         field_name: "statusId",
    //         field_value: "EXST_IN_DESIGN"
    //     }) - string_store.id(SerialKey{
    //         entity_name: "ExampleStatus",
    //         field_name: "exampleId",
    //         field_value: "EX01"
    //     })
    // );

    let mut serials=Vec::new();
    for (entry,val) in &string_store.string_store {
        serials.push((val, entry));
    }
    let json_str=serde_json::to_string_pretty(&serials)?;
    println!("{}", json_str);

    string_store.store();
    Ok(())
}

#[test]
fn load_works()  -> anyhow::Result<()>{
    let string_store=StringStore::load()?;
    for (k,v) in &string_store.string_store{
        println!("{:?}, {}", k, v);
    }
    Ok(())
}

#[test]
fn transform_works() {
    use chrono::format::strftime::StrftimeItems;

    let parse_dt=NaiveDateTime::parse_from_str;
    let fmt = StrftimeItems::new("%Y-%m-%dT%H:%M:%S");
    let mut store=StringStore::load().unwrap();

    // let _ = simple_logger::init();
    let xml_str=r##"<ExampleStatusSeed exampleId="EX01" statusDate="2007-04-05T14:30:30"
    statusEndDate="2007-04-05T14:30:30" statusId="EXST_IN_DESIGN"/>"##;
    let data:ExampleStatusSeed=serde_xml_rs::from_str(xml_str).unwrap();
    println!("{:?}", data);

    let xml_str=r##"<ExampleStatus exampleId="EX01" statusDate="2010-01-02 00:00:00"
    statusEndDate="2011-01-02 00:00:00" statusId="EXST_IN_DESIGN"/>"##;
    let doc = roxmltree::Document::parse(xml_str).unwrap();
    let node=doc.root_element();
    let status_dt=node.attribute("statusDate").unwrap();
    println!("{} -> {} {}", node.tag_name().name(),
             status_dt,
             parse_dt(status_dt, "%Y-%m-%d %H:%M:%S").unwrap()
                 .format_with_items(fmt.clone()).to_string()
    );
    let flds:Vec<String>=node.attributes().iter().map(|f| {
        let mut fld_val:String=f.value().to_string();
        if f.name().ends_with("Date"){
            fld_val=parse_dt(f.value(), "%Y-%m-%d %H:%M:%S").unwrap()
                .format_with_items(fmt.clone()).to_string();

        } else if f.name().ends_with("Id") {
            let val=store.id(SerialKey::new("ExampleStatus", f.name(), f.value()));
            fld_val=val.to_string();
        }
        format!(" {}=\"{}\"", f.name(), fld_val)
    }).collect();
    let node_str=format!("<{} {}/>", node.tag_name().name(), flds.join(" "));
    println!("{}", node_str);

    let data:ExampleStatus=serde_xml_rs::from_str(node_str.as_str()).unwrap();
    println!("{:?}", data);

    store.store().unwrap();
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum TestSeed {
    #[serde(rename_all = "camelCase")]
    UserLogin {
        user_login_id: String,
        enabled: Option<String>,
        nick_name: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Appveyor {
        repository: String,
        id: Option<String>,
        branch: Option<String>,
        project_name: Option<String>,
        service: Option<String>,
    },
}

#[test]
fn security_entity_works() -> anyhow::Result<()> {
    // use crate::models::security::UserLogin;
    let node_str=r##"<UserLogin userLoginId="188" enabled="N"/>"##;
    let data:TestSeed=serde_xml_rs::from_str(node_str).unwrap();
    println!("{:?}", data);

    let json_str=r#"{"UserLogin":{"userLoginId":"0","enabled":"N", "other":0}}"#;
    let data:TestSeed=serde_json::from_str(json_str)?;
    println!("{:?}", data);
    Ok(())
}

// use decimal::prelude::*;
use serde::de::DeserializeOwned;
use crate::snowflake::new_snowflake_id;
use crate::load_xml;

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

