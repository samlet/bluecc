use serde_json::json;
use crate::{GenericError, load_xml};
use glob::{MatchOptions, glob_with};
use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::private::PathAsDisplay;
use std::str::FromStr;
use std::str;
use super::*;
use serde::{Serialize, de};
use crate::meta_model::{EntityModel, Entity};
use roxmltree::Node;
use crate::meta::service_models::{ServiceModel, ModelService};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataFile{
    pub uri: String,
    pub path: String,
    pub content: String,
    #[serde(default)]
    pub items: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataFiles{
    pub files: Vec<DataFile>,
}

pub fn list_files(dir: &str, pattern: &str) -> Result<Vec<PathBuf>, GenericError>{
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let mut rs:Vec<PathBuf>=Vec::new();
    for entry in glob_with(
        format!("{}/{}", dir, pattern).as_str(), options)? {
        let path = entry?;
        // rs.push(path.to_str().unwrap().to_string());
        rs.push(path.to_owned());
    }
    Ok(rs)
}

pub fn merge_files(dir: &str, filters: Vec<&str>, json_output: &str, file_type: &FileTypes)
    -> Result<String,GenericError>{

    use std::io::prelude::*;

    let mut data_files=DataFiles{ files: vec![] };
    let mut files:Vec<PathBuf>=Vec::new();
    for filter in &filters {
        files.append(&mut list_files(dir, filter)?);
    }
    for f in &files{
        debug!(".. read {} start", f.as_display());
        let cnt=std::fs::read_to_string(f)?;
        // println!(".. read {} end", f.as_display());
        let path=f.to_str().unwrap().to_owned();
        let items=get_items_in_file(path.as_str(), &file_type)?.iter()
                .map(|e|e.clone())
                .collect::<Vec<String>>();
        if !items.is_empty() {
            data_files.files.push(DataFile {
                uri: f.file_name().unwrap().to_str().unwrap().to_string(),
                path: path.to_owned(),
                content: cnt,
                items: items,
            });
        }
    }

    let zout=store_z(&data_files, json_output)?;
    Ok(zout)
}

pub fn store_z<T>(data_files:&T, json_output: &str) -> Result<String,GenericError>
where
    T: ?Sized + Serialize,{
    use std::io::prelude::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    let val=serde_json::to_string_pretty(&data_files)?;
    std::fs::write(json_output, &val)?;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&val.as_bytes());
    let compressed_bytes = e.finish()?;
    let zout=format!("{}z", json_output);
    std::fs::write(&zout, compressed_bytes)?;
    Ok(zout)
}


pub fn load_z<T>(bytes:&[u8]) -> Result<T, GenericError>
where
    T: de::DeserializeOwned,{
    use std::io::prelude::*;
    use flate2::read::ZlibDecoder;

    let mut d = ZlibDecoder::new(bytes);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    debug!("size {}", &s.len());
    let data_files:T=serde_json::from_str(&s)?;

    Ok(data_files)
}

pub struct ServiceModelReader{
    data_files: DataFiles,
    cached_srvs: HashMap<String, ModelService>,
}
impl ServiceModelReader{
    pub fn new() -> Result<Self, GenericError> {
        // let bytes =std::fs::read("./.store/service_model_files.jsonz")?;
        let bytes=include_bytes!("pkgs/service_model_files.jsonz");
        let data_files=load_z::<DataFiles>(bytes)?;
        Ok(ServiceModelReader { data_files: (data_files), cached_srvs:HashMap::new() })
    }

    pub fn get_all_service_names(&self)->Vec<String>{
        self.data_files.files.iter().flat_map(|f|f.items.clone()).collect::<Vec<String>>()
    }

    pub fn load_all_srvs(&mut self) -> Result<Vec<&ModelService>, GenericError> {
        for f in &self.data_files.files {
            let model: ServiceModel = load_xml(f.content.as_bytes());
            for s in model.services{
                self.cached_srvs.insert(s.name.to_string(), s);
            }
        }
        let vals=Vec::from_iter(self.cached_srvs.values());
        Ok(vals)
    }

    pub fn get_service_model(&mut self, srv_name: &str) -> Result<&ModelService, GenericError> {
        if !self.cached_srvs.contains_key(srv_name) {
            for f in &self.data_files.files {
                if f.items.contains(&srv_name.to_string()) {
                    // let model: ServiceModel = load_xml(f.content.as_bytes());
                    let model: ServiceModel = load_xml(f.content.as_bytes());
                    // let item = model.services.iter()
                    //     .filter(|e| e.name == srv_name)
                    //     .nth(0);
                    // self.cached_srvs.insert(srv_name.to_string(), item.unwrap().clone());

                    // put all services which defined in this service-model-file into cache
                    for s in model.services{
                        self.cached_srvs.insert(s.name.to_string(), s);
                    }
                    break;
                }
            }
        }

        if !self.cached_srvs.contains_key(srv_name) {
            Err(GenericError::NotFound {
                item_name: srv_name.to_string(),
                info: "no such service".to_string()
            })
        }else{
            Ok(self.cached_srvs.get(srv_name).unwrap())
        }
    }
}

pub struct ModelReader{
    data_files: DataFiles,
    cached_ents: HashMap<String, Entity>,
}

fn fix_module_name(module_name: &str) -> String {
    let mut target=module_name.to_string();
    if !target.ends_with(".xml"){
        target.push_str("-entitymodel.xml");
    }
    target
}

impl ModelReader{
    pub fn load() -> Result<Self, GenericError> {
        // let bytes =std::fs::read("./.store/entity_model_files.jsonz")?;
        let bytes=include_bytes!("pkgs/entity_model_files.jsonz");
        Ok(ModelReader {
            data_files: (load_z::<DataFiles>(bytes)?),
            cached_ents: HashMap::new(),
        })
    }

    pub fn get_entity_module(&self, module_name: &str) -> Result<EntityModel, GenericError> {
        let target=fix_module_name(module_name);
        for f in &self.data_files.files {
            if f.uri==target {
                let mut model: EntityModel = load_xml(f.content.as_bytes());
                model.build();
                return Ok(model);
            }
        }
        Err(GenericError::NotFound {
            item_name: target,
            info: "entity module".to_string()
        })
    }

    pub fn get_all_entity_names(&self)->Vec<String>{
        self.data_files.files.iter().flat_map(|f|f.items.clone()).collect::<Vec<String>>()
    }

    pub fn get_entity_model(&mut self, entity_name: &str) -> Result<Entity, GenericError> {
        if let Some(ent)=self.cached_ents.get(entity_name){
            return Ok(ent.clone());
        }
        for f in &self.data_files.files {
            if f.items.contains(&entity_name.to_string()) {
                let mut model: EntityModel = load_xml(f.content.as_bytes());
                model.build();
                let ent = &model.entities.iter()
                    .filter(|e| e.entity_name == entity_name)
                    .nth(0);
                for e in &model.entities{
                    self.cached_ents.insert(e.entity_name.to_owned(), e.clone());
                }
                return Ok(ent.expect("entity").clone());
            }
        }

        Err(GenericError::NotFound {
            item_name: entity_name.to_string(),
            info: "entity model".to_string()
        })
    }
}

pub struct SeedFiles{
    pub data_files: DataFiles,
}
impl SeedFiles{
    pub fn load() -> Result<Self,GenericError> {
        let bytes=include_bytes!("pkgs/seed_files.jsonz");
        let data_files=load_z::<DataFiles>(bytes)?;
        Ok(SeedFiles { data_files })
    }

    pub fn entity_seeds(&self, entity_name: &str) -> Result<Vec<HashMap<String,String>>, GenericError> {
        let mut result=Vec::new();
        for f in &self.data_files.files {
            if f.items.contains(&entity_name.to_string()) {
                let doc = roxmltree::Document::parse(f.content.as_str())?;
                let nodes = &doc.descendants()
                    .filter(|e| e.has_tag_name(entity_name))
                    .collect::<Vec<Node<'_, '_>>>();
                debug!("doc {} has {} {}", f.uri, nodes.len(), entity_name);
                for n in nodes {
                    let m: HashMap<_, _>= n.attributes().into_iter()
                        .map(|a|(a.name().to_owned(), a.value().to_owned())).collect();
                    result.push(m);
                }
            }
        }

        Ok(result)
    }
}

pub fn load_seed_model_z_file<P>(entity_name: &str, proc: P) -> Result<(), GenericError>
where P: Fn(&Node<'_,'_>) -> bool,{
    // let bytes =std::fs::read("./.store/seed_files.jsonz")?;
    let bytes=include_bytes!("pkgs/seed_files.jsonz");
    let data_files=load_z::<DataFiles>(bytes)?;
    for f in &data_files.files{
        if f.items.contains(&entity_name.to_string()){
            let doc = roxmltree::Document::parse(f.content.as_str())?;
            let nodes=doc.descendants()
                .filter(|e|e.has_tag_name(entity_name))
                .collect::<Vec<Node<'_,'_>>>();
            info!("doc {} has {} {}", f.uri, nodes.len(), entity_name);
            for n in nodes{
                if !proc(&n){
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn json_object_works() -> anyhow::Result<()> {
        use crate::models::security_types::SecurityGroup;
        let json = json!(
            {
              "groupId": 212118821551607808_i64,
              "groupName": "Full Admin",
              "description": "Full Admin group, has all general permissions."
            }
        );
        let rec = serde_json::from_value::<SecurityGroup>(json)?;
        println!("{:?}", rec);
        Ok(())
    }

    #[test]
    fn list_files_works() -> anyhow::Result<()> {
        let files = list_files("./data", "**/*.xml")?;
        for f in &files {
            println!("{:?}: {}", f.file_name().unwrap().to_string_lossy(), f.as_display());
        }

        let f = files.get(0).unwrap();
        let cnt = std::fs::read_to_string(f)?;
        println!("{}", cnt);
        Ok(())
    }

    #[test]
    fn merge_data_files_works() -> anyhow::Result<()> {
        let zout = merge_files("./data", vec!["**/*.xml"],
                               "./.store/data_files.json", &FileTypes::Data)?;
        println!("save to {}", zout);
        Ok(())
    }

    #[test]
    fn merge_seed_data_files_works() -> anyhow::Result<()> {
        let dir = &cc_conf()?.get_ofbiz_root();
        let zout = merge_files(dir, vec!["**/data/*.xml"],
                               "./.store/seed_files.json", &FileTypes::Data)?;
        println!("save to {}", zout);
        Ok(())
    }

    #[test]
    fn merge_entity_models_works() -> anyhow::Result<()> {
        let zout = merge_files("./entitydef", vec!["**/*.xml"],
                               "./.store/entity_model_files.json", &FileTypes::EntityModel)?;
        println!("save to {}", zout);
        Ok(())
    }

    #[test]
    fn merge_service_models_works() -> anyhow::Result<()> {
        let dir = &cc_conf()?.get_ofbiz_root();
        let zout = merge_files(dir, vec!["**/servicedef/*.xml"],
                               "./.store/service_model_files.json", &FileTypes::ServiceModel)?;
        println!("save to {}", zout);
        Ok(())
    }

    #[test]
    fn load_z_works() -> anyhow::Result<()> {
        let bytes: &[u8] = include_bytes!("fixtures/data_files.jsonz");
        let data_files = load_z::<DataFiles>(bytes)?;
        for f in &data_files.files {
            println!("{}: {}", f.uri, f.path);
        }
        Ok(())
    }

    #[test]
    fn load_z_file_works() -> anyhow::Result<()> {
        let bytes = std::fs::read("./.store/data_files.jsonz")?;
        let data_files = load_z::<DataFiles>(&bytes)?;
        for f in &data_files.files {
            println!("{}: {}", f.uri, f.path);
            println!("\t{:?}", f.items);
        }
        Ok(())
    }

    #[test]
    fn load_entity_model_z_file_works() -> anyhow::Result<()> {
        // let bytes =std::fs::read("./.store/entity_model_files.jsonz")?;
        let bytes = include_bytes!("pkgs/entity_model_files.jsonz");
        let data_files = load_z::<DataFiles>(bytes)?;
        let entity_name = "Example";
        for f in &data_files.files {
            if f.items.contains(&entity_name.to_string()) {
                let mut model: EntityModel = load_xml(f.content.as_bytes());
                model.build();
                let ent = model.entities.iter()
                    .filter(|e| e.entity_name == entity_name)
                    .nth(0);
                let ent_json = serde_json::to_string_pretty(ent.unwrap())?;
                println!("{}", ent_json);
            }
        }
        Ok(())
    }

    #[test]
    fn model_reader_works() -> anyhow::Result<()> {
        let mut reader = ModelReader::load()?;
        let ent = reader.get_entity_model("Example")?;
        let ent_json = serde_json::to_string_pretty(&ent)?;
        println!("{}", ent_json);
        Ok(())
    }

    #[test]
    fn load_seed_model_z_file_works() -> Result<(), GenericError> {
        load_seed_model_z_file("Person", |n| {
            println!("{} ({:?})", n.tag_name().name(), n.range());
            for attr in n.attributes() {
                println!("\t{} = {}", attr.name(), attr.value());
            }
            true
        })?;
        Ok(())
    }

    #[test]
    fn load_service_model_z_file_works() -> anyhow::Result<()> {
        let srv_name = "createExample";
        let mut sr = ServiceModelReader::new()?;
        let item = sr.get_service_model(srv_name)?;
        let json_str = serde_json::to_string_pretty(&item)?;
        println!("{} => {}", srv_name, json_str);
        Ok(())
    }
}
