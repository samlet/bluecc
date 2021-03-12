#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut, unused_must_use, deprecated))]

pub mod entity_xml_tests;
mod collection_tests;
mod string_tests;
mod cell_tests;
mod globals;
mod meta_tests;
#[cfg(test)]
mod yml_tests;
mod sql_tests;
mod entity_xml_parse_tests;
mod model_tests;
#[cfg(test)]
mod redis_tests;
#[cfg(test)]
mod scale_tests;
mod template_tests;
#[cfg(test)]
mod functor_tests;
#[cfg(test)]
mod linq_tests;
mod seed_gen;
mod requests;

pub mod app_context;
mod service_models;
mod resource_loader;
mod askama_tests;
mod form_parsers;
mod basic_models;
#[cfg(test)]
mod test_helpers;
pub mod seed_conf;
#[cfg(test)]
mod serializer_tests;
mod cc_conf;
mod security_procs;

