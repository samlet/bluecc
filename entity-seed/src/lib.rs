#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lazy_static_include;
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
mod delegators;
mod kube;

pub use self::database::establish_connection;
pub use self::util::deserialize_branch_with_contiguous_check as load_xml;
pub use self::errors::GenericError;
pub use self::meta::app_context::{get_entity_by_name, get_entity_model,
                                  security_model, example_model, get_entities_by_module_names};
pub use self::snowflake::new_snowflake_id;

#[cfg(test)]
mod lib_tests {
    use crate::security_model;

    #[test]
    fn entity_meta_works() {
        let model=security_model();
        assert!(model.get_entity("UserLogin")
            .get_field("userLoginId").unwrap().is_id_type());
    }
}



