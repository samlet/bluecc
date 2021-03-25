use serde::{Serialize, Deserialize, de};
use std::collections::HashMap;
use crate::GenericError;
use seed::load_xml;
use seed::meta::{ModelService, ServiceModel, CC_CONF};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentModel{
    pub name: String,
    #[serde(rename = "resource-loader", default)]
    pub resource_loader: Vec<ResourceLoader>,
    #[serde(rename = "classpath", default)]
    pub classpaths: Vec<Classpath>,
    #[serde(rename = "entity-resource", default)]
    pub entity_resources: Vec<ModelResource>,
    #[serde(rename = "service-resource", default)]
    pub service_resources: Vec<ModelResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceLoader{
    pub name: String,
    #[serde(rename = "type", default)]
    pub loader_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Classpath{
    #[serde(rename = "type", default)]
    pub type_name: String,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelResource{
    #[serde(rename = "type", default)]
    pub type_name: String,
    #[serde(rename = "reader-name", default)]
    pub reader_name: String,
    pub loader: String,
    pub location: String,
}

impl ComponentModel{
    pub fn load() -> Result<Self, GenericError>{
        let conf_path= CC_CONF.get_component_conf_path();
        debug!("read from {}", conf_path);
        let content=std::fs::read(conf_path)?;
        let model:ComponentModel=load_xml(&*content);
        Ok(model)
    }

    pub fn load_all_services(&self) -> Result<HashMap<String,ModelService>, GenericError>{
        let mut srvs=HashMap::new();
        let root=CC_CONF.get_srv_root();
        for srv_res in &self.service_resources {
            let srv_path=format!("{}/{}", root, srv_res.location);
            let cnt=std::fs::read_to_string(srv_path)?;
            let model: ServiceModel = load_xml(cnt.as_bytes());

            for s in model.services {
                srvs.insert(s.name.to_string(), s);
            }
        }
        Ok(srvs)
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use envmnt::{ExpandOptions, ExpansionType};

    #[test]
    fn model_works() -> anyhow::Result<()> {
        let conf_path= CC_CONF.get_component_conf_path();
        println!("read from {}", conf_path);
        let content=std::fs::read(conf_path)?;
        let model:ComponentModel=load_xml(&*content);
        println!("{}", model.name);
        Ok(())
    }

    #[test]
    fn env_works() -> anyhow::Result<()> {
        let mut options = ExpandOptions::new();
        options.expansion_type = Some(ExpansionType::Unix);
        let mut value = envmnt::expand("ofbiz root is: ${HOME}/ofbiz", Some(options));
        println!("Expanded: {}", &value);
        Ok(())
    }

    #[test]
    fn service_resources_works() -> anyhow::Result<()> {
        let comps=ComponentModel::load()?;
        let srvs=comps.load_all_services()?;
        println!("{:?}", srvs.keys());
        Ok(())
    }
}

