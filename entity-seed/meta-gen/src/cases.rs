use serde_json::json;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};
use tera::{Context, Tera};
use crate::{DynamicValue, GenericError, SrvResp};
use deles::delegators::pretty;
use itertools::Itertools;
use reqwest::Client;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cases{
    pub resources: Vec<CaseResource>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CaseResource{
    /// Id representing the resource
    #[serde(rename = "_id", default)]
    pub id: String,
    /// Resource ID of parent object (folder or workspace)
    pub parent_id: String,
    /// When the resource was last modified
    pub modified: u64,
    /// When the resource was created
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

impl Cases{
    fn get_children_names(&self, root_name: &str) -> Vec<String>{
        let empty="_".to_string();
        let srv_root=self.resources.iter()
            .find(|&r|Some(root_name.to_string())==r.name).unwrap();
        let children=self.resources.iter().filter(|r|r.parent_id==srv_root.id)
            .collect_vec();
        let child_names=children.iter()
            .map(|&n|n.name.as_ref().unwrap_or(&empty).to_owned())
            .collect::<Vec<String>>();
        child_names
    }
}

impl CaseResource{
    pub async fn request(&self, client:&Client, vars:&str) -> crate::Result<SrvResp<DynamicValue>> {
        use reqwest::{header, StatusCode as Status};

        // setup vars
        let mut values:HashMap<String, String>= serde_json::from_str(vars)?;
        let orders=vec!["ofbiz_base", "ofbiz_rest", "ofbiz_srvs"];

        expand_vars(&mut values, &orders)?;
        let mut context = Context::new();
        context.insert("_", &values);
        let expand_var=|val| {
            Tera::one_off(val, &context, false).unwrap()
        };

        // expand vars and do request
        let body=self.body.as_ref().unwrap().text.as_str();
        let req_data:DynamicValue=serde_json::from_str(&expand_var(body))?;
        debug!("{}", pretty(&req_data));

        let url=self.url.as_ref().unwrap();
        let token=&expand_var(self.authentication.as_ref().unwrap().token.as_str());
        debug!("token {}", token);
        let res = client
            .post(&expand_var(url.as_str()))
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(&req_data)
            .send()
            .await?;

        // process response
        debug!("result -> {} {:?}", res.status(), res);
        let data = res.json::<SrvResp<DynamicValue>>().await?;
        Ok(data)
    }
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

pub fn list_related_srvs(srv_name:&str) -> crate::Result<()> {
    let cases_data = include_str!("cases/cases_ofbiz.yaml");
    let cases: Cases = serde_yaml::from_str(cases_data)?;
    let rels=cases.resources.iter()
        .filter(|r|r.url.is_some() && r.url.as_ref().unwrap().ends_with(srv_name))
        .collect_vec();
    for rel in rels{
        println!("{}", rel.id);
        println!("{}", rel.body.as_ref().unwrap().text);
    }
    Ok(())
}

#[cfg(test)]
mod lib_tests {
    use super::*;

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

    #[test]
    fn case_names_works() -> anyhow::Result<()> {
        let cases_data=include_str!("cases/Insomnia_2021-03-29.yaml");
        let cases:Cases=serde_yaml::from_str(cases_data)?;
        let empty="_".to_string();
        let names=cases.resources.iter()
            .map(|r|(r.name.as_ref().unwrap_or(&empty), &r.type_name))
            .collect_vec();
        println!("{}", pretty(&names));

        let srv_root=cases.resources.iter()
            .find(|&r|Some("ofbiz-srvs".to_string())==r.name).unwrap();
        let children=cases.resources.iter().filter(|r|r.parent_id==srv_root.id)
            .collect_vec();
        let child_names=children.iter()
            .map(|&n|n.name.as_ref().unwrap_or(&empty))
            .collect_vec();
        println!("{}'s children: {}", srv_root.name.as_ref().unwrap(), pretty(&child_names));

        Ok(())
    }

    #[test]
    fn get_children_names_works() -> anyhow::Result<()> {
        let cases_data=include_str!("cases/Insomnia_2021-03-29.yaml");
        let cases:Cases=serde_yaml::from_str(cases_data)?;
        println!("{:?}", cases.get_children_names("ofbiz-srvs"));
        Ok(())
    }

    #[test]
    fn get_related_srvs_works() -> anyhow::Result<()> {
        let cases_data = include_str!("cases/cases_ofbiz.yaml");
        let cases: Cases = serde_yaml::from_str(cases_data)?;
        let srv_name="updatePartyEmailAddress";
        let rels=cases.resources.iter()
            .filter(|r|r.url.is_some() && r.url.as_ref().unwrap().ends_with(srv_name))
            .collect_vec();
        for rel in rels{
            println!("{}", rel.id);
            println!("{}", rel.body.as_ref().unwrap().text);
        }
        Ok(())
    }

    #[tokio::test]
    async fn request_works() -> crate::Result<()> {
        use reqwest::{header, StatusCode as Status, Client};

        let cases_data=include_str!("cases/Insomnia_2021-03-29.yaml");
        let cases:Cases=serde_yaml::from_str(cases_data)?;
        let item=cases.resources.get(0).unwrap();
        println!("{}", pretty(item));

        let vars=include_str!("cases/env_vars.json");

        // setup vars
        let mut values:HashMap<String, String>= serde_json::from_str(vars)?;
        let orders=vec!["ofbiz_base", "ofbiz_rest", "ofbiz_srvs"];

        expand_vars(&mut values, &orders)?;
        let expand_var=|val| {
            let mut context = Context::new();
            context.insert("_", &values);
            let t = Tera::one_off(val, &context, false).unwrap();
            t
        };

        // expand vars and do request
        let body=item.body.as_ref().unwrap().text.as_str();
        let req_data:DynamicValue=serde_json::from_str(&expand_var(body))?;
        println!("{}", pretty(&req_data));

        let mut client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build().unwrap();

        let url=item.url.as_ref().unwrap();
        let token=&expand_var(item.authentication.as_ref().unwrap().token.as_str());
        println!("token {}", token);
        let res = client
            .post(&expand_var(url.as_str()))
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .header(header::ACCEPT, "application/json")
            .json(&req_data)
            .send()
            .await?;

        // process response
        println!("result -> {} {:?}", res.status(), res);

        let data = res.json::<SrvResp<DynamicValue>>().await?;
        let data_json = serde_json::to_string_pretty(&data)?;
        println!("data -> {}", data_json);

        Ok(())
    }

    #[tokio::test]
    async fn resource_request_works() -> crate::Result<()> {
        let cases_data=include_str!("cases/Insomnia_2021-03-29.yaml");
        let cases:Cases=serde_yaml::from_str(cases_data)?;
        let item=cases.resources.get(0).unwrap();
        // println!("{}", pretty(item));

        let vars=include_str!("cases/env_vars.json");

        let mut client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build().unwrap();

        let resp=item.request(&client, vars).await?;
        println!("{}", pretty(&resp));

        Ok(())
    }
}


