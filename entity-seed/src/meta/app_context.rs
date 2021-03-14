use std::str;
use serde_xml_rs::from_str;
use crate::meta_model::*;
use std::collections::HashMap;
use crate::load_xml;

lazy_static! {
    pub static ref APP_CONTEXT:AppContext={
        AppContext::new()
    };
}

lazy_static_include_bytes! {
    SECURITY_DOC => "entitydef/security-entitymodel.xml",
    COMMON_DOC => "entitydef/common-entitymodel.xml",
    EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
    ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
}

fn get_field_mappings() -> FieldTypes{
    // from_str(str::from_utf8(include_bytes!("fixtures/fieldtypemysql.xml")).unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("fixtures/fieldtypepostgres_saas.xml")).unwrap()).unwrap()
}
// fn example_models() -> EntityModel{
//     from_str(str::from_utf8(include_bytes!("fixtures/entitymodel_example.xml")).unwrap()).unwrap()
// }

pub struct AppContext{
    models: HashMap<String,EntityModel>,
    pub field_mappings: FieldTypes
}

impl AppContext{
    pub fn new() -> Self {
        let mut resources: HashMap<String, EntityModel> =
            [("common".to_string(), load_xml(&**COMMON_DOC)),
                ("security".to_string(), load_xml(&**SECURITY_DOC)),
                ("example".to_string(), load_xml(&**EXAMPLE_DOC)),
                ("accounting".to_string(), load_xml(&**ACCOUNTING_DOC)),]
                .iter().cloned().collect();
        for model in resources.values_mut() {
            model.build();
        }
        AppContext { models:resources, field_mappings:get_field_mappings()  }
    }

    pub fn get_model(&self, name: &str) -> &EntityModel{
        &self.models.get(name).expect(format!("cannot find entity module {}", name).as_str())
    }
}

pub fn get_entity_model(name: &str) ->  &'static EntityModel{
    APP_CONTEXT.get_model(name)
}

pub fn get_entity_by_name(entity_name: &str) -> Option<&Entity> {
    APP_CONTEXT.models.values().into_iter()
        .flat_map(|e|&e.entities)
        .filter(|e| e.entity_name==entity_name)
        .nth(0)
}
pub fn get_entities_by_module_names(mods:&Vec<String>) -> Vec<String> {
    APP_CONTEXT.models.iter()
        .filter(|(k,_)|mods.contains(k))
        .flat_map(|(_, e)|e.entity_names())
        .collect()
}

pub fn security_model() ->  &'static EntityModel{
    get_entity_model("security")
}

pub fn example_model() ->  &'static EntityModel{
    get_entity_model("example")
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn get_ent_meta_works() -> anyhow::Result<()> {
        let ent=get_entity_by_name("UserLogin");
        assert_eq!("UserLogin", ent.unwrap().entity_name);
        Ok(())
    }
}