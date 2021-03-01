// extern crate serde;
// extern crate serde_xml_rs;
use serde_xml_rs::{from_reader, from_str};
use std::str;
use itertools::Itertools;
use phf::{phf_map};
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    pub name: String,
    pub source: String
}

#[derive(Debug, Deserialize)]
struct Project {
    pub name: String,

    #[serde(rename = "Item", default)]
    pub items: Vec<Item>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity{
    #[serde(rename = "entity-name", default)]
    pub entity_name: String,
    #[serde(rename = "field", default)]
    pub fields: Vec<ModelField>,
    #[serde(rename = "relation", default)]
    pub relations: Vec<ModelRelation>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelField{
    #[serde(rename = "name", default)]
    pub field_name: String,
    #[serde(rename = "type", default)]
    pub field_type: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRelation{
    #[serde(rename = "type", default)]
    pub rel_type: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityModel{
    pub title: String,
    pub description: String,
    pub version: String,
    #[serde(rename = "default-resource-name", default)]
    pub default_resource_name: String,
    #[serde(rename = "entity", default)]
    pub entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldTypeDef{
    #[serde(rename = "type", default)]
    pub field_type: String,
    #[serde(rename = "sql-type", default)]
    pub sql_type: String,
    #[serde(rename = "java-type", default)]
    pub java_type: String
}
#[derive(Debug, Deserialize)]
pub struct FieldTypes{
    #[serde(rename = "field-type-def", default)]
    pub field_types: Vec<FieldTypeDef>
}

#[test]
fn new_works() {
    let test_values = [b'A', b'B', b'C', b'D', b'E', b'F'];
    assert_eq!(test_values.is_empty(), false);
}

#[test]
fn embed_string() {
    let my_str = include_str!("spanish.in");
    assert_eq!(my_str, "adiós\n");
    print!("{}", my_str);
}

// ref: https://crates.io/crates/serde-xml-rs
#[test]
fn xml_string_works() {
    let s = r##"
        <Project name="my_project">
            <Item name="hello" source="world.rs" />
        </Project>
    "##;
    let project: Project = from_reader(s.as_bytes()).unwrap();
    println!("{:#?}", project);

    let s = r##"<item name="hello" source="world.rs" />"##;
    let item: Item = from_reader(s.as_bytes()).unwrap();
    assert_eq!(item, Item { name: "hello".to_string(),source: "world.rs".to_string()});
}

pub fn example_model() -> EntityModel{
    let s = r##"
        <entitymodel xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
            xsi:noNamespaceSchemaLocation="http://ofbiz.apache.org/dtds/entitymodel.xsd">
            <!-- ========================================================= -->
            <!-- ======================== Defaults ======================= -->
            <!-- ========================================================= -->
            <title>Entity of an Apache OFBiz Component</title>
            <description>None</description>
            <version>1.0</version>
            <default-resource-name>ExampleEntityLabels</default-resource-name>
            <entity entity-name="Example" package-name="org.apache.ofbiz.example.example" title="Example Entity">
                <field name="exampleId" type="id"><description>primary sequenced ID</description></field>
                <field name="exampleTypeId" type="id"></field>
                <field name="statusId" type="id"></field>
                <field name="exampleName" type="name"></field>
                <field name="description" type="description"></field>
                <field name="longDescription" type="very-long"></field>
                <field name="comments" type="comment"></field>
                <field name="exampleSize" type="numeric"></field>
                <field name="exampleDate" type="date-time"></field>
                <field name="anotherDate" type="date-time"></field>
                <field name="anotherText" type="long-varchar"></field>
                <prim-key field="exampleId"/>
                <relation type="one" fk-name="EXMPL_TYP" rel-entity-name="ExampleType">
                    <key-map field-name="exampleTypeId"/>
                </relation>
                <relation type="one" fk-name="EXMPL_STTS" rel-entity-name="StatusItem">
                    <key-map field-name="statusId"/>
                </relation>
            </entity>
        </entitymodel>
    "##;
    from_reader(s.as_bytes()).unwrap()
}

pub fn example_models() -> EntityModel{
    // from_reader(include_bytes!("entitymodel_example.xml").unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("entitymodel_example.xml")).unwrap()).unwrap()
}

impl EntityModel {
    pub fn get_entity(&self, name: &str) -> &Entity {
        self.entities.iter().find(|n|n.entity_name==name).expect("find entity")
    }
}

#[test]
fn entity_model_works() {
    let model: EntityModel = example_model();
    println!("{:#?}", model);
    let ent=model.entities.iter().find(|n|n.entity_name=="Example").unwrap();
    for f in &ent.fields{
        println!("{}", f.field_name);
    }
}

lazy_static! {
    pub static ref FIELD_MAPPINGS:FieldTypes=get_field_mappings();
}
#[test]
fn entity_gen_works() {
    use tera::{Result, Context, Filter, Function};
    use tera::Tera;
    use serde_json::{json, Value};

    struct SqlType;
    impl Filter for SqlType {
        fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val=FIELD_MAPPINGS.sql_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }

        fn is_safe(&self) -> bool {
            true
        }
    }
    fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=inflector::cases::snakecase::to_snake_case(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }

    let model=example_model();
    let ent=model.get_entity("Example");
    assert_eq!("Example", ent.entity_name);

    let mut tera = Tera::default();
    tera.add_raw_template(
        "ent",
        r#"
CREATE TABLE {{ent['entity-name'] | snake_case -}} (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
{%- for fld in flds %}
    {{fld.name | snake_case}}: {{fld['type'] | sqltype}},
{%- endfor %}
);
        "#,
    )
        .unwrap();

    let mut context = Context::new();
    tera.register_filter("sqltype", SqlType);
    tera.register_filter("snake_case", snake_case);
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields);
    let result = tera.render("ent", &context);
    println!("{}", result.unwrap());
}

#[test]
fn fields_works() {
    let model: EntityModel = from_str(
        str::from_utf8(include_bytes!("entitymodel_example.xml")).unwrap()).unwrap();
    println!("{:#?}", model.title);
    let ent=model.entities.iter().find(|n|n.entity_name=="Example").unwrap();
    let names:Vec<String>=ent.fields.iter().
        map(|n| n.field_name.clone()).collect();
    println!("{:?}", names.iter().join(", "));

    let data_formatter = ent.fields.iter()
        .format_with(",\n", |elt, f|
            f(&format_args!("pub {}: {}", elt.field_name, elt.field_type)));
    println!("{}", data_formatter);
}

pub fn get_field_mappings() -> FieldTypes{
    from_str(str::from_utf8(include_bytes!("fieldtypemysql.xml")).unwrap()).unwrap()
}
impl FieldTypes{
    pub fn sql_type(&self, field_type:&str) -> String{
        self.field_types.iter()
            .find(|x| x.field_type==field_type)
            .unwrap().sql_type.clone()
    }
}

#[test]
fn field_mapping_works() {
    let model:FieldTypes=from_str(str::from_utf8(include_bytes!("fieldtypemysql.xml")).unwrap()).unwrap();
    let ft=model.field_types.iter().find(|x| x.field_type=="id");
    println!("{}", ft.expect("not find").sql_type);
}

static COUNTRIES: phf::Map<&'static str, &'static str> = phf_map! {
    "US" => "United States",
    "UK" => "United Kingdom",
};

#[test]
fn phf_works() {
    println!("{}", COUNTRIES.get("US").unwrap_or(&"_"));
    println!("{}", COUNTRIES.get("us").unwrap_or(&"_"));
}