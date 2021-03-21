use seed::{EntityGenerator, ModelField, FIELD_MAPPINGS};
use seed::meta::{SeedFiles, load_seed_model_z_file};
use crate::{GenericError, ServiceMeta};
use std::collections::HashMap;
use tera::Context;
use serde_json::Value;

pub trait MetaGenerator{
    fn generate_for(&mut self, template: &str, ent_name: &str)
        -> Result<String, GenericError>;
}

fn plain_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let mut val:String = FIELD_MAPPINGS.orig_type(value.as_str().unwrap());
    if val.starts_with("Option") {
        val=val.chars().skip_while(|&c|c!='<')
            .skip(1)
            .take_while(|&c|c!='>').collect();
    }
    Ok(Value::String(format!("{}", val)))
}

impl MetaGenerator for ServiceMeta {
    fn generate_for(&mut self, template: &str, ent_name: &str) -> Result<String, GenericError> {
        let ent = self.entity_reader.get_entity_model(ent_name)?;
        let flds=&ent.fields;

        let mut generator = EntityGenerator::new(vec![ent_name.to_string()]);
        generator.tera.register_filter("plain_type", plain_type);

        generator.tera.add_raw_template("value_obj",
                                        include_str!("incls/value_obj.j2"))?;
        generator.tera.add_raw_template("ink",
                                        include_str!("incls/ink_obj.j2"))?;

        let mut context = Context::new();
        context.insert("ent", &ent);
        context.insert("flds", flds);
        let result = generator.tera.render(template, &context)?;

        Ok(result)
    }
}

