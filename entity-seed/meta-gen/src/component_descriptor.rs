use serde::{Serialize, Deserialize, de};
use std::collections::HashMap;
use crate::GenericError;
use seed::{load_xml, EntityModel, Entity, FIELD_MAPPINGS};
use seed::meta::{ModelService, ServiceModel, CcConfig, CC_CONF};
use seed::{EntityGenerator, ModelField};
use crate::ParamMode;

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

pub struct ComponentDescriptor{
    pub conf: CcConfig,
    pub model: ComponentModel,
}

impl ComponentDescriptor{
    pub fn load(conf: &CcConfig) -> Result<Self, GenericError>{
        let conf_path= conf.get_component_conf_path();
        debug!("read from {}", conf_path);
        let content=std::fs::read(conf_path)?;
        let model:ComponentModel=load_xml(&*content);
        Ok(ComponentDescriptor{ conf: conf.clone(), model })
    }

    pub fn load_all_services(&self) -> Result<HashMap<String,ModelService>, GenericError>{
        let mut srvs=HashMap::new();
        let root=self.conf.get_srv_root();
        let model_res=self.model.service_resources.iter()
            .filter(|r|r.type_name=="model")
            .collect::<Vec<&ModelResource>>();
        for srv_res in &model_res {
            let srv_path=format!("{}/{}", root, srv_res.location);
            let cnt=std::fs::read_to_string(srv_path)?;
            let model: ServiceModel = load_xml(cnt.as_bytes());

            for s in model.services {
                srvs.insert(s.name.to_string(), s);
            }
        }
        Ok(srvs)
    }

    pub fn load_all_entities(&self) -> Result<HashMap<String, Entity>, GenericError>{
        let mut ents=HashMap::new();
        let root=self.conf.get_srv_root();
        let model_res=self.model.entity_resources.iter()
            .filter(|r|r.type_name=="model")
            .collect::<Vec<&ModelResource>>();
        for res in &model_res {
            let srv_path=format!("{}/{}", root, res.location);
            info!("load resource file {}", srv_path);
            // let cnt=std::fs::read_to_string(srv_path)?;
            // let model: EntityModel = load_xml(cnt.as_bytes());
            let model: EntityModel = EntityModel::load(srv_path.as_str())?;

            for e in model.entities {
                ents.insert(e.entity_name.to_string(), e);
            }
        }
        Ok(ents)
    }
}

pub fn get_srv(comp_name:&str, srv_name:&str) -> Result<(ModelService, HashMap<String, Entity>), GenericError> {
    let conf=CcConfig{
        ofbiz_loc: CC_CONF.ofbiz_loc.to_owned(),
        srv_root: comp_name.to_string() };
    let comps=ComponentDescriptor::load(&conf)?;
    let srvs=comps.load_all_services()?;
    let ents=comps.load_all_entities()?;
    let srv=srvs.get(srv_name).expect("srv-model").to_owned();
    Ok( (srv, ents))
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use envmnt::{ExpandOptions, ExpansionType};
    use crate::{ServiceMeta, ParamMode, ModelParam};
    use tera::Context;
    use itertools::Itertools;
    use inflector::Inflector;

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
        let comps=ComponentDescriptor::load(&CC_CONF)?;
        let srvs=comps.load_all_services()?;
        println!("{:?}", srvs.keys());
        Ok(())
    }

    #[test]
    fn ex_service_resources_works() -> anyhow::Result<()> {
        let conf=CcConfig{
            ofbiz_loc: CC_CONF.ofbiz_loc.to_owned(),
            srv_root: "plugins/example".to_string() };
        let comps=ComponentDescriptor::load(&conf)?;
        let srvs=comps.load_all_services()?;
        println!("{:?}", srvs.keys());
        Ok(())
    }

    #[test]
    fn entity_resources_works() -> anyhow::Result<()> {
        let comps=ComponentDescriptor::load(&CC_CONF)?;
        let ents=comps.load_all_entities()?;
        println!("{:?}", ents.keys());
        Ok(())
    }

    #[test]
    fn ex_entity_resources_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "info,entity_seed=debug");
        env_logger::init();

        let conf=CcConfig{
            ofbiz_loc: CC_CONF.ofbiz_loc.to_owned(),
            srv_root: "plugins/example".to_string() };
        let comps=ComponentDescriptor::load(&conf)?;
        println!("{:?}", comps.model.entity_resources);
        let ents=comps.load_all_entities()?;
        println!("{:?}", ents.keys());
        Ok(())
    }

}

