use serde_json::json;
use linked_hash_map::LinkedHashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cases{
    pub resources: Vec<CaseResource>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CaseResource{
    #[serde(rename = "_id", default)]
    pub id: String,
    pub parent_id: String,
    pub modified: u64,
    pub created: u64,
    pub url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub method: Option<String>,
    pub body: Option<ResourceBody>,
    #[serde(default)]
    pub parameters: Vec<String>,
    #[serde(default)]
    pub headers: Vec<ResourceHeader>,
    pub authentication: Option<Authentication>,
    #[serde(rename = "_type", default)]
    pub type_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authentication{
    pub token: String,
    #[serde(rename = "type", default)]
    pub type_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceBody{
    pub mime_type: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResourceHeader{
    pub name: String,
    pub value: String,
    pub id: String,
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    use serde::Deserialize;
    use std::{collections::HashMap, fs::File};
    use tera::{Context, Tera};
    use crate::{DynamicValue, GenericError};
    use deles::delegators::pretty;

    #[derive(Debug, Deserialize)]
    struct Config {
        boolean: bool,
        float: f32,
        map: HashMap<u8, char>,
        nested: Nested,
        tuple: (u32, u32),
        vec: Vec<Nested>,
    }

    #[derive(Debug, Deserialize)]
    struct Nested {
        a: String,
        b: char,
    }

    #[test]
    fn ron_works() {
        let input_path = format!("{}/test_files/test.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&input_path).expect("Failed opening file");
        let config: Config = ron::de::from_reader(f).unwrap();
        println!("Config: {:?}", &config);
    }

    #[test]
    fn one_off_works() -> anyhow::Result<()> {
        let mut context = Context::new();
        context.insert("greeting", &"hello");
        let r=Tera::one_off("{{ greeting }} world", &context, true)?;
        println!("{}", r);
        Ok(())
    }

    #[derive(Serialize)]
    struct Test {
        a: String,
        b: String,
        c: Vec<String>,
    }

    #[test]
    fn var_access_works() -> anyhow::Result<()> {
        let mut context = Context::new();
        context.insert("_", &Test { a: "hi".into(), b: "there".into(), c: vec![] });
        let t = Tera::one_off("{{ _.a }}", &context, true)?;
        println!("{}", t);
        Ok(())
    }

    fn expand_vars(values:&mut HashMap<String, String>, orders:&Vec<&str>) -> Result<(), GenericError>{
        let mut context = Context::new();
        context.insert("_", &values);
        for &key in orders{
            let val=values.get(key).unwrap();
            let t = Tera::one_off(val, &context, false)?;
            values.insert(key.to_string(), t.to_owned());
            context.insert("_", &values);  // refresh context
        }
        Ok(())
    }

    #[test]
    fn expand_table_works() -> anyhow::Result<()> {
        let mut values: HashMap<String, String> = serde_json::from_value(json!({
                      "ofbiz_base": "https://localhost:8443",
                      "ofbiz_rest": "{{ _.ofbiz_base }}/rest"
                }))?;
        let orders = vec!["ofbiz_base", "ofbiz_rest"];
        expand_vars(&mut values, &orders)?;
        println!("{}", pretty(&values));
        Ok(())
    }

    #[test]
    fn expand_file_works() -> anyhow::Result<()> {
        let vars=include_str!("cases/env_vars.json");
        let mut values:HashMap<String, String>= serde_json::from_str(vars)?;
        let orders=vec!["ofbiz_base", "ofbiz_rest", "ofbiz_srvs"];
        expand_vars(&mut values, &orders)?;
        println!("{}", pretty(&values));
        Ok(())
    }

    #[test]
    fn cases_works() -> anyhow::Result<()> {
        let cases_data=include_str!("cases/Insomnia_2021-03-29.yaml");
        let cases:Cases=serde_yaml::from_str(cases_data)?;
        let item=cases.resources.get(0).unwrap();
        println!("{}", pretty(item));

        if let Some(body)=&item.body{
            println!("{}", body.text);

            let req:DynamicValue=serde_json::from_str(body.text.as_str())?;
            println!("{}", pretty(&req));
        }
        Ok(())
    }
}


