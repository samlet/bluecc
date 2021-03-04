use std::str;
use serde_xml_rs::from_str;
use crate::meta_model::*;

lazy_static! {
    pub static ref APP_CONTEXT:AppContext={
        AppContext::new()
    };
}

fn get_field_mappings() -> FieldTypes{
    // from_str(str::from_utf8(include_bytes!("fieldtypemysql.xml")).unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("fieldtypepostgres.xml")).unwrap()).unwrap()
}
fn example_models() -> EntityModel{
    // from_reader(include_bytes!("entitymodel_example.xml").unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("entitymodel_example.xml")).unwrap()).unwrap()
}

pub struct AppContext{
    pub models: EntityModel,
    pub field_mappings: FieldTypes
}

impl AppContext{
    pub fn new() -> Self {
        let mut models: EntityModel=example_models();
        for mut ent in &mut models.entities {
            for mut fld in &mut ent.fields {
                let is_pk=match ent.primary_keys.iter().map(|x| x.field_name.clone())
                    .find(|f|f==&fld.field_name) {
                    Some(_f) => true,
                    _ => false
                };
                fld.is_primary=is_pk;
            }
            ent.multiple_keys=ent.primary_keys.len()>1;
        }
        AppContext { models, field_mappings:get_field_mappings()  }
    }
}

