// use serde_xml_rs::{from_reader, from_str};
use std::str;
use std::io::prelude::*;
use crate::meta_model::EntityModel;
use serde::Deserialize;
use std::io::{Read, BufReader};
use std::collections::{HashMap, BTreeMap, HashSet};
use chrono::Utc;
use serde_json::{Value, Error};
use crate::util::{parse_pair};
use std::fs;

// use decimal::prelude::*;
use serde::de::DeserializeOwned;
use crate::snowflake::new_snowflake_id;
use crate::{load_xml, get_entity_model, GenericError, get_entity_by_name};
use roxmltree::Node;
use std::fs::read_to_string;
use crate::models::enum_types::EntityTypes;
use chrono::{NaiveDateTime, NaiveDate};
use crate::models::model_types::SeedTypes;
use crate::meta::cc_conf::CcConfig;
use glob::{MatchOptions, glob_with};

use super::*;

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
    pub static ref SKIP_NODES:Vec<&'static str>=vec!["create", "create-replace", "create-update", "delete"];
}

pub fn skip_nodes() -> &'static Vec<&'static str>{
    return &SKIP_NODES;
}

pub enum FileTypes{
    Data,
    EntityModel,
    ServiceModel
}

pub fn get_items_in_file(xml_file: &str, file_type: &FileTypes) -> Result<HashSet<String>, GenericError> {
    let xml_str=std::fs::read_to_string(xml_file)?;
    let mut doc = roxmltree::Document::parse(xml_str.as_str()).unwrap();
    let root = doc.root_element();

    match file_type {
        FileTypes::Data => {
            if root.has_tag_name("entity-engine-xml") {
                let excepts = vec!["create", "create-replace", "create-update", "delete"];
                let ents = root.children().into_iter()
                    .filter(|e| e.is_element() && !excepts.contains(&e.tag_name().name()))
                    .map(|e| e.tag_name().name().to_string())
                    .collect::<HashSet<String>>();

                Ok(ents)
            }else{
                Ok(HashSet::new())
            }
        }
        FileTypes::EntityModel =>{
            if root.has_tag_name("entitymodel") {
                // let node_types=vec!["entity", "view-entity"];
                let node_types = vec!["entity"];
                let ents = root.children().into_iter()
                    .filter(|e| e.is_element() && node_types.contains(&e.tag_name().name()))
                    .map(|e| e.attribute("entity-name").expect("entity-node").to_string())
                    .collect::<HashSet<String>>();

                Ok(ents)
            }else{
                Ok(HashSet::new())
            }
        }
        FileTypes::ServiceModel =>{
            let node_types=vec!["service"];
            let srvs = root.children().into_iter()
                .filter(|e| e.is_element() && node_types.contains(&e.tag_name().name()))
                .map(|e| e.attribute("name").expect("service-node").to_string())
                .collect::<HashSet<String>>();

            Ok(srvs)
        }
        // _ => Ok(HashSet::new())
    }

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

    pub fn save(&self, file_name: &str) -> Result<(), GenericError>{
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

    pub fn store(&self) -> Result<(), GenericError>{
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

fn process_seed(xml_str: &str) -> Result<Vec<SeedTypes>, GenericError>{
    use chrono::format::strftime::StrftimeItems;

    let parse_dt=NaiveDateTime::parse_from_str;
    let fmt = StrftimeItems::new("%Y-%m-%dT%H:%M:%S");
    let mut store=StringStore::load().unwrap();

    let doc = roxmltree::Document::parse(xml_str).unwrap();
    let root=doc.root_element();
    let nodes=root.children()
        .filter(|n| n.is_element()).collect::<Vec<Node<'_,'_>>>();
    // let model=get_entity_model("security");

    let mut result_set=Vec::new();
    for node in nodes{
        let node_name=node.tag_name().name();
        let mod_ent=get_entity_by_name(node_name).expect("entity");
        let flds:Vec<String>=node.attributes().iter().map(|f| {
            let fld_name=f.name();
            let mut fld_val:String=f.value().to_string();
            let mod_fld=mod_ent.get_field(fld_name).expect("field");
            if mod_fld.is_dt_type() {
                fld_val=parse_dt(f.value(), "%Y-%m-%d %H:%M:%S%.f").unwrap()
                    .format_with_items(fmt.clone()).to_string();

            } else if mod_fld.is_id_type() {
                let mut rel_ent=node_name.to_string();
                match mod_ent.get_relation_entity(fld_name) {
                    Some(r) => rel_ent=r,
                    _ => ()
                }
                let val=store.id(SerialKey::new(rel_ent.as_str(), f.name(), f.value()));
                fld_val=val.to_string();
            } else if mod_fld.field_type=="indicator" {
                fld_val= match fld_val.as_str() {
                    "Y" => "true".to_string(),
                    "N" => "false".to_string(),
                    _ => "false".to_string()
                };
            }
            format!(" {}=\"{}\"", f.name(), fld_val)
        }).collect();

        let node_str=format!("<{} {}/>", node.tag_name().name(), flds.join(" "));
        let data:SeedTypes=serde_xml_rs::from_str(node_str.as_str()).unwrap();
        println!("{} ->\n  {:?}", node_str, data);
        result_set.push(data);
    }

    store.store()?;
    Ok(result_set)
}

#[test]
fn process_seed_works() -> anyhow::Result<()> {
    let cnt=read_to_string("data/security/SecurityGroupDemoData.xml")?;
    let rs=process_seed(cnt.as_str())?;
    println!("total {}", rs.len());
    match rs.get(0) {
        Some(SeedTypes::SecurityGroup(e))=> {
            println!("group id {:?}\n{}", e.group_id, serde_json::to_string_pretty(e)?);
            // store it to db
        }
        _ => ()
    }
    Ok(())
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

#[test]
fn entity_types_works() -> anyhow::Result<()> {
    // use crate::models::security::UserLogin;
    let node_str=r##"<UserLogin userLoginId="188" enabled="true"/>"##;
    let data:EntityTypes=serde_xml_rs::from_str(node_str).unwrap();
    match data {
        EntityTypes::UserLogin {enabled, .. } => {assert!(enabled.unwrap())}
        _ => ()
    }
    println!("{:?}", data);

    let json_str=r#"{"UserLogin":{"userLoginId":0,"enabled":false, "other":0}}"#;
    let data:EntityTypes=serde_json::from_str(json_str)?;
    match data {
        EntityTypes::UserLogin {enabled, .. } => {assert!(!enabled.unwrap())}
        _ => ()
    }
    println!("{:?}", data);
    Ok(())
}

#[test]
fn seed_types_works() -> anyhow::Result<()> {
    use crate::models::security_types::{UserLogin, UserLoginPasswordHistory};
    #[derive(Deserialize, Debug)]
    struct Seeds {
        items: Vec<Types>
    }
    #[derive(Deserialize, Debug)]
    enum Types {
        UserLogin(UserLogin),
        UserLoginPasswordHistory(UserLoginPasswordHistory)
    }

    let node_str = r##"<UserLogin userLoginId="188" enabled="true"/>"##;
    let data: Types = serde_xml_rs::from_str(node_str).unwrap();
    match data {
        Types::UserLogin(ref rec) => { assert!(rec.enabled.unwrap()) }
        _ => ()
    }
    println!("{:?}", data);
    Ok(())
}


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

pub fn list_data_files() -> anyhow::Result<()> {
    let config=cc_conf()?;
    println!("ofbiz location: {}", config.ofbiz_loc);

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    for entry in glob_with(
        format!("{}/**/data/*.xml", config.ofbiz_loc).as_str(), options)? {
        let path=entry?;
        println!("{}", path.display());
        let ents= get_items_in_file(path.to_str().unwrap(), &FileTypes::Data)?;
        let r=serde_json::to_string_pretty(&ents)?;
        println!("{}", r)
    }

    Ok(())
}

#[test]
fn list_data_files_works() -> anyhow::Result<()> {
    list_data_files()
}