#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, unused_must_use, deprecated))]

#[cfg(test)]
mod entity_xml_tests;
#[cfg(test)]
mod collection_tests;
#[cfg(test)]
mod string_tests;
#[cfg(test)]
mod cell_tests;
#[cfg(test)]
mod yml_tests;
#[cfg(test)]
mod sql_tests;
#[cfg(test)]
mod entity_xml_parse_tests;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod redis_tests;
#[cfg(test)]
mod scale_tests;
#[cfg(test)]
mod template_tests;
#[cfg(test)]
mod functor_tests;
// #[cfg(test)]
// mod linq_tests;
#[cfg(test)]
mod askama_tests;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod serializer_tests;
#[cfg(test)]
mod regex_tests;

mod seed_gen;
mod requests;

pub mod app_context;
mod service_models;
pub mod resource_loader;

mod basic_models;
mod form_parsers;
pub mod seed_conf;
pub mod cc_conf;
mod security_procs;
mod data_files_procs;
pub mod model_revisions;

pub use resource_loader::{get_items_in_file, skip_nodes, FileTypes};
pub use data_files_procs::{merge_files, ModelReader, ServiceModelReader, load_seed_model_z_file};
pub use cc_conf::{cc_conf};
pub use service_models::{ModelService, ServiceImplements,
                         ServiceAutoAttributes, ServiceModel};

