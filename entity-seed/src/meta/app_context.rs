use std::str;
use serde_xml_rs::from_str;
use crate::meta_model::*;
use std::collections::HashMap;
use crate::{load_xml, GenericError};
use crate::meta::ModelReader;
use std::sync::Mutex;

lazy_static! {
    pub static ref APP_CONTEXT:Mutex<AppContext>={
        Mutex::new(AppContext::new())
    };
    pub static ref FIELD_MAPPINGS:FieldTypes={
        get_field_mappings()
    };
}
//
// lazy_static_include_bytes! {
//     SECURITY_DOC => "entitydef/security-entitymodel.xml",
//     COMMON_DOC => "entitydef/common-entitymodel.xml",
//     EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
//     ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
// }

fn get_field_mappings() -> FieldTypes{
    // from_str(str::from_utf8(include_bytes!("fixtures/fieldtypemysql.xml")).unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("fixtures/fieldtypepostgres_saas.xml")).unwrap()).unwrap()
}

pub struct AppContext{
    // models: HashMap<String,EntityModel>,
    pub reader: ModelReader,
    // pub field_mappings: FieldTypes,
}

impl AppContext{
    pub fn new() -> Self {
        // let mut resources: HashMap<String, EntityModel> =
        //     [("common".to_string(), load_xml(&**COMMON_DOC)),
        //         ("security".to_string(), load_xml(&**SECURITY_DOC)),
        //         ("example".to_string(), load_xml(&**EXAMPLE_DOC)),
        //         ("accounting".to_string(), load_xml(&**ACCOUNTING_DOC)),]
        //         .iter().cloned().collect();
        // for model in resources.values_mut() {
        //     model.build();
        // }
        // AppContext { models:resources, field_mappings:get_field_mappings()  }
        AppContext { reader:ModelReader::load().expect("load entity models")  }
    }

    pub fn get_model(&mut self, name: &str) -> Result<Entity, GenericError>{
        // &self.models.get(name).expect(format!("cannot find entity module {}", name).as_str())
        self.reader.get_entity_model(name)
    }
}

pub fn get_entity_model(name: &str) -> Result<Entity, GenericError> {
    APP_CONTEXT.lock().unwrap().get_model(name)
}

pub fn get_entity_module(name: &str) -> Result<EntityModel, GenericError>{
    APP_CONTEXT.lock().unwrap().reader.get_entity_module(name)
}


// pub fn get_entity_by_name(entity_name: &str) -> Option<&Entity> {
//     APP_CONTEXT.models.values().into_iter()
//         .flat_map(|e|&e.entities)
//         .filter(|e| e.entity_name==entity_name)
//         .nth(0)
// }

pub fn get_entities_by_module_names(mods:&Vec<String>) -> Vec<String> {
    let models:Vec<EntityModel>=mods.into_iter()
        .map(|m| get_entity_module(m).expect("get entity"))
        .collect();
    models.iter()
        .flat_map(|e|e.entity_names())
        .collect()
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn get_ent_meta_works() -> anyhow::Result<()> {
        let ent=get_entity_model("UserLogin")?;
        assert_eq!("UserLogin", ent.entity_name);
        Ok(())
    }
}
