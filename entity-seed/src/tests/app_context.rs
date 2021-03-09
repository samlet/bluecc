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
    COMMON_DOC => "entitydef/entitymodel.xml",
    EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
    ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
}

fn get_field_mappings() -> FieldTypes{
    // from_str(str::from_utf8(include_bytes!("fieldtypemysql.xml")).unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("fieldtypepostgres_saas.xml")).unwrap()).unwrap()
}
// fn example_models() -> EntityModel{
//     from_str(str::from_utf8(include_bytes!("entitymodel_example.xml")).unwrap()).unwrap()
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


