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

    fn get_srv(comp_name:&str, srv_name:&str) -> Result<(ModelService, HashMap<String, Entity>), GenericError> {
        let conf=CcConfig{
            ofbiz_loc: CC_CONF.ofbiz_loc.to_owned(),
            srv_root: comp_name.to_string() };
        let comps=ComponentDescriptor::load(&conf)?;
        let srvs=comps.load_all_services()?;
        let ents=comps.load_all_entities()?;
        let srv=srvs.get(srv_name).expect("srv-model").to_owned();
        Ok( (srv, ents))
    }

    fn srv_param_type(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let mut type_name:String = value.as_str().unwrap().to_string();
        if !type_name.is_pascal_case() {
            type_name = FIELD_MAPPINGS.orig_type(type_name.as_str());
        }
        Ok(tera::Value::String(type_name))
    }

    fn guess_action(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let mut type_name:String = value.as_str().unwrap().to_string();
        let mut action=type_name.to_pascal_case();
        if type_name.starts_with("create") {
            action="Create".to_string();
        }else if type_name.starts_with("update") {
            action="Update".to_string();
        }else if type_name.starts_with("delete") || type_name.starts_with("remove") {
            action="Delete".to_string();
        }
        Ok(tera::Value::String(action))
    }

    #[test]
    fn srv_gen_works() -> anyhow::Result<()> {
        let (srv,ents)=get_srv("plugins/example", "createExample")?;
        println!("srv name {} with {}, ents: {:?}", srv.name, srv.default_entity_name, ents.keys());
        let params=ServiceMeta::srv_model_params(&srv, &ents)?;

        for p in &params{
            println!("{} {:?} {}", p.name, p.mode, p.optional);
        }

        let inputs=params.iter()
            .filter(|p|p.mode==ParamMode::In || p.mode==ParamMode::InOut)
            .collect::<Vec<&ModelParam>>();
        let outputs=params.iter()
            .filter(|p|p.mode==ParamMode::Out || p.mode==ParamMode::InOut)
            .collect::<Vec<&ModelParam>>();
        println!("outputs: {}", outputs.len());

        // generate
        let mut generator = EntityGenerator::new(ents.keys().cloned().collect());
        // generator.tera.register_filter("param_type", srv_param_type);
        generator.tera.register_filter("action", guess_action);
        generator.tera.add_raw_template("srv_create", include_str!("incls/srv_create.j2"))?;
        generator.tera.add_raw_template("srv_resp", include_str!("incls/srv_resp.j2"))?;

        let mut context = Context::new();
        context.insert("srv", &srv);
        if !srv.default_entity_name.is_empty(){
            context.insert("ent", ents.get(srv.default_entity_name.as_str()).unwrap());
        }
        context.insert("inputs", &inputs);
        context.insert("outputs", &outputs);

        let result = generator.tera.render("srv_create", &context)?;
        println!("result => \n{}", result);

        let result = generator.tera.render("srv_resp", &context)?;
        println!("result => \n{}", result);
        Ok(())
    }
}

