#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

mod seed_gen;
mod conn;
mod entity_auto_procs;
mod custom_types;
mod service_gen;
mod requests;
mod user_login;
mod value_objs;
mod component_descriptor;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use]
extern crate diesel;

pub use requests::{SrvResp, SrvErr, SrvDeles, DynamicValue};
pub use service_gen::{ServiceMeta, ParamMode, ModelParam};
pub use seed::{GenericError};
pub use user_login::{UserWithPassword};
pub use value_objs::{Generator};

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


