#![feature(in_band_lifetimes)]
#![allow(dead_code, unused_imports, unused_mut)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

mod seed_gen;
mod conn;
mod entity_auto_procs;
mod custom_types;
mod service_gen;
mod requests;
mod user_login;
mod value_objs;
mod component_descriptor;
mod params;
mod functions;
mod resource_gen;
mod srv_example;
mod srv_finders;
mod resources;
pub mod cases;
mod xml_seed;
mod meta_conf;
pub mod solidity_gen;
mod proto_gen;
mod java_gen;
mod exec_plan;
mod entity_info;
mod status_item;
mod meta_services;
mod chart_gen;
mod charts;
mod action_meta;
mod capnp_gen;
mod active_units;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use]
extern crate diesel;

pub use requests::{SrvResp, SrvErr, SrvDeles, DynamicValue, extract_val};
pub use service_gen::{ServiceMeta, ParamMode, ModelParam};
pub use seed::{GenericError};
pub use user_login::{UserWithPassword};
pub use value_objs::{Generator};
pub use component_descriptor::{ComponentDescriptor, ComponentModel,
                               get_srv};
pub use resource_gen::{generate_srv_invoker, generate_srv_ent};
pub use proto_gen::{generate_for_proto};
pub use xml_seed::{process_seed};
pub use meta_conf::{META_CONF};
pub use entity_info::{pprint_tree};
pub use status_item::{StateGraph};
pub use chart_gen::{ChartGen};

mod error{
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MetaError {
        #[error("io error")]
        Io(#[from] std::io::Error),
        #[error("parse error")]
        Parse(#[from] std::num::ParseIntError),
        #[error("xml parse fail")]
        ParseXml(#[from] roxmltree::Error),
        #[error("time error")]
        SystemTimeErr(#[from] std::time::SystemTimeError),
        #[error("json parse fail")]
        JsonSerialize(#[from] serde_json::Error),
        #[error("xml parse fail")]
        XmlSerialize(#[from] serde_xml_rs::Error),
        #[error("yaml parse fail")]
        YamlSerialize(#[from] serde_yaml::Error),
        #[error("request fail")]
        RequestErr(#[from] reqwest::Error),
        #[error("redis fail")]
        RedisErr(#[from] redis::RedisError),
        #[error("tera template fail")]
        TeraTemplateErr(#[from] tera::Error),
        #[error("config toml error")]
        ConfigTomlErr(#[from] toml::de::Error),
        #[error("generic error")]
        GenericErr(#[from] seed::GenericError),
        #[error("service error")]
        ServiceErr(#[from] deles::ServiceError),
    }
}

pub type Result<T> = std::result::Result<T, error::MetaError>;

#[cfg(test)]
mod tests {
    use seed::{new_snowflake_id, get_entity_model};
    use seed::meta::ServiceModelReader;

    const STORE_FILE: &str ="../.store/id_store.json";

    #[test]
    fn it_works() {
        use std::path::Path;
        assert!(Path::new(STORE_FILE).exists());
    }

    #[test]
    fn id_works() -> anyhow::Result<()> {
        let new_id=new_snowflake_id();
        println!("new id {}", new_id);
        Ok(())
    }

    #[test]
    fn entity_meta_works() -> anyhow::Result<()> {
        let ent=get_entity_model("Person")?;
        println!("{:?}", ent);
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


