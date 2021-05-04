use serde_json::json;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};
use tera::{Context, Tera};
use crate::{DynamicValue, GenericError, SrvResp};
use deles::delegators::pretty;
use itertools::Itertools;
use reqwest::Client;

pub struct CasesManager{
    cases: Cases,
    cases_by_id_map: HashMap<String, usize>,

    cases_file: String,
    // tree: CaseTree,
    workloads: Vec<Workload>,
}

#[derive(Debug, Serialize)]
pub struct Workload{
    name: String,
    id: String,
    index: usize,
    states: Vec<WorkState>,
}

#[derive(Debug, Serialize)]
pub struct WorkState{
    name: String,
    id: String,
    index: usize,
    actions: Vec<usize>,
}

impl CasesManager{
    pub fn load() -> crate::Result<Self> {
        use std::{env, fs};

        let target_dir = dirs::home_dir().unwrap();
        let target_dir = target_dir.join("Downloads/fixtures");
        let mut files=Vec::new();
        for entry in fs::read_dir(target_dir)? {
            let entry = entry?;
            let path = entry.path();

            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();

            files.push((path.as_path().to_string_lossy().to_string().to_owned(), last_modified));
        }

        files.sort_by(|a,b| a.1.cmp(&b.1));
        for (f,ts) in &files {
            debug!("{:?} {}", f, ts);
        }
        let latest=files.first().unwrap().0.to_owned();
        debug!("the latest file: {}", latest);

        // parse as cases
        let bytes=std::fs::read(latest.as_str())?;
        let cases:Cases=serde_yaml::from_reader(&*bytes)?;
        debug!("total cases {}", cases.resources.len());

        let workloads:Vec<&CaseResource>=cases.resources.iter()
            .filter(|r|r.type_name=="request_group" && r.name.starts_with("workload:"))
            .collect();
        let cases_by_id_map:HashMap<String, usize>=cases.resources.iter()
            .enumerate()
            .map(|(i, r)|(r.id.to_string(), i))
            .collect();
        let mut workload_rs=Vec::new();
        for workload in &workloads{
            debug!("{}: {}", workload.id, workload.name);
            let mut work_states=Vec::new();
            let states:Vec<&CaseResource>=cases.resources.iter()
                .filter(|r|r.parent_id==workload.id && r.type_name=="request_group")
                .collect();
            for st in &states{
                debug!("\t - {}: {}", st.id, st.name);
                let actions:Vec<usize>=cases.resources.iter()
                    .filter(|r|r.parent_id==st.id)
                    .map(|r|*cases_by_id_map.get(r.id.as_str()).unwrap())
                    .collect();
                work_states.push(WorkState{
                    name: st.name.to_string(),
                    id: st.id.to_string(),
                    index: *cases_by_id_map.get(st.id.as_str()).unwrap(),
                    actions
                });
            }

            workload_rs.push(Workload{
                name: workload.name.to_string(),
                id: workload.id.to_string(),
                index: *cases_by_id_map.get(workload.id.as_str()).unwrap(),
                states: work_states,
            });
        }

        Ok(CasesManager{ cases, cases_by_id_map,
            cases_file:latest.to_owned(),
            workloads:workload_rs,
        })
    }

    pub fn workload_by_name(&self, name: &str) -> Option<&Workload> {
        let full_name=format!("workload:{}", name);
        self.workloads.iter().find(|w|w.name==full_name)
    }

    pub fn workload_names(&self) -> Vec<String>{
        self.workloads.iter()
            .map(|f|f.name.strip_prefix("workload:")
                .expect("trim prefix").to_string())
            .collect_vec()
    }

    pub fn cases(&self) -> &Cases{
        &self.cases
    }

    pub fn print_workload(&self, workload_name:&str){
        let workload= self.workload_by_name(workload_name);
        if let Some(w)=workload {
            println!("{}", pretty(w));
            // let st=w.states.get(0).unwrap();
            for st in &w.states {
                let cases = self.cases();
                let act_names = st.actions.iter().map(|a| cases.resource(*a))
                    .map(|r| r.name.to_string())
                    .collect_vec();
                println!("{} => \n {:?}", st.name, act_names);
            }
        }
    }

    pub fn print_action(&self, workload_name: &str, act_name: &str){
        let workload= self.workload_by_name(workload_name);
        fn extract(body: &Option<ResourceBody>) -> String {
            if let Some(r)=body{
                r.text.to_owned()
            }else{
                "".to_string()
            }
        }
        if let Some(w)=workload {
            for st in &w.states {
                let cases = self.cases();
                let acts = st.actions.iter()
                    .map(|a| cases.resource(*a))
                    .filter(|a|a.name==act_name)
                    .map(|a| (a.name.to_string(), extract(&a.body)))
                    .collect_vec();
                if !acts.is_empty() {
                    println!("{} => ", st.name);
                    for (name, code) in &acts{
                        println!("{}", name);
                        println!("{}", code);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cases{
    pub resources: Vec<CaseResource>,
}

impl Cases{
    pub fn resource(&self, idx: usize) -> &CaseResource {
        self.resources.get(idx).expect("index")
    }
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
    #[serde(default)]
    pub name: String,
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
    #[serde(default)]
    pub token: String,
    #[serde(rename = "type", default)]
    pub type_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourceBody{
    #[serde(default)]
    pub mime_type: String,
    #[serde(default)]
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
        // let empty="_".to_string();
        let srv_root=self.resources.iter()
            .find(|&r|root_name.to_string()==r.name).unwrap();
        let children=self.resources.iter().filter(|r|r.parent_id==srv_root.id)
            .collect_vec();
        let child_names=children.iter()
            .map(|&n|n.name.to_owned())
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
        // let empty="_".to_string();
        let names=cases.resources.iter()
            .map(|r|(r.name.to_owned(), &r.type_name))
            .collect_vec();
        println!("{}", pretty(&names));

        let srv_root=cases.resources.iter()
            .find(|&r|"ofbiz-srvs".to_string()==r.name).unwrap();
        let children=cases.resources.iter().filter(|r|r.parent_id==srv_root.id)
            .collect_vec();
        let child_names=children.iter()
            .map(|&n|n.name.to_owned())
            .collect_vec();
        println!("{}'s children: {}", srv_root.name, pretty(&child_names));

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

    #[tokio::test]
    async fn workload_list_works() -> anyhow::Result<()> {
        use std::{env, fs};

        let target_dir = dirs::home_dir().unwrap();
        let target_dir = target_dir.join("Downloads/fixtures");
        let mut files=Vec::new();
        for entry in fs::read_dir(target_dir)? {
            let entry = entry?;
            let path = entry.path();

            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();

            // if last_modified < 24 * 3600 && metadata.is_file() {
            //     println!(
            //         "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
            //         last_modified,
            //         metadata.permissions().readonly(),
            //         metadata.len(),
            //         path.file_name().expect("No filename")
            //     );
            // }
            files.push((path.as_path().to_string_lossy().to_string().to_owned(), last_modified));
        }
        files.sort_by(|a,b| a.1.cmp(&b.1));
        for (f,ts) in &files {
            println!("{:?} {}", f, ts);
        }
        let latest=files.first().unwrap().0.to_string();
        println!("the latest file: {}", latest);

        // parse as cases
        let bytes=std::fs::read(latest)?;
        let cases:Cases=serde_yaml::from_reader(&*bytes)?;
        println!("total cases {}", cases.resources.len());

        let workloads:Vec<&CaseResource>=cases.resources.iter()
            .filter(|r|r.type_name=="request_group" && r.name.starts_with("workload:"))
            .collect();
        for workload in &workloads{
            println!("{}: {}", workload.id, workload.name);
            let states:Vec<&CaseResource>=cases.resources.iter()
                .filter(|r|r.parent_id==workload.id && r.type_name=="request_group")
                .collect();
            for st in &states{
                println!("\t - {}: {}", st.id, st.name);
            }
        }

        Ok(())
    }

    #[test]
    fn workload_names_works() -> anyhow::Result<()> {
        let cases_mgr=CasesManager::load()?;
        let names=cases_mgr.workload_names();
        println!("{:?}", names);

        Ok(())
    }

    #[test]
    fn load_states_works() -> anyhow::Result<()> {
        let cases_mgr=CasesManager::load()?;
        // let workload=cases_mgr.workloads.get(0).unwrap();
        cases_mgr.print_workload("Example");

        Ok(())
    }

     #[test]
    fn print_state_action_works() -> anyhow::Result<()> {
        let cases_mgr=CasesManager::load()?;
        cases_mgr.print_action("Example", "createExampleStatus");

        Ok(())
    }
}


