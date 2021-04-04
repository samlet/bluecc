#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, deprecated))]
#![recursion_limit="256"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate lazy_static_include;
#[macro_use]
extern crate diesel;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
#[macro_use] extern crate log;
// #[macro_use]
// extern crate error_chain;

pub mod meta;
pub mod meta_model;
pub mod util;
pub mod schema;
pub mod models;
pub mod database;
pub mod errors;
pub mod snowflake;
mod object_id;
mod topo;
pub mod stopwatch;

pub use self::database::establish_connection;
pub use self::util::deserialize_branch_with_contiguous_check as load_xml;
pub use self::errors::GenericError;
pub use self::meta::app_context::{get_entity_model, get_entity_module, APP_CONTEXT, FIELD_MAPPINGS};
pub use self::snowflake::new_snowflake_id;
pub use self::meta::resource_loader::{SeedProcessor, StringStore, SerialKey};
pub use self::meta::model_revisions::{Revisions};
pub use self::meta::template_builder::EntityGenerator;
pub use self::models::model_types::{SeedTypes};
pub use self::meta_model::{Entity, EntityModel, ModelField, ModelRelation, KeyMap};

pub fn exists(file: &str) -> bool{
    use std::path::Path;
    Path::new(file).exists()
}

#[cfg(test)]
mod lib_tests {
    use crate::get_entity_model;

    #[test]
    fn entity_meta_works() -> anyhow::Result<()>{
        assert!(get_entity_model("UserLogin")?
            .get_field("userLoginId").unwrap().is_id_type());
        Ok(())
    }
}



