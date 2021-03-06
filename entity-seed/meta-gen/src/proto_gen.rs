use seed::{EntityGenerator, ModelField, FIELD_MAPPINGS};
use std::collections::HashMap;
use tera::Context;
use serde_json::Value;
use inflector::Inflector;
use itertools::Itertools;

fn proto_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let val:String = FIELD_MAPPINGS.proto_type(value.as_str().unwrap());
    Ok(Value::String(format!("{}", val)))
}

pub fn generate_for_proto(template: &str, ent_name: &str) -> crate::Result<String> {
    let ent:seed::Entity = seed::get_entity_model(ent_name)?;
    // let flds = &ent.fields;

    let mut generator = EntityGenerator::new(vec![ent_name.to_string()]);
    generator.tera.register_filter("proto_type", proto_type);
    generator.tera.add_raw_template("proto", include_str!("incls/proto_obj.j2"))?;

    let mut context = tera::Context::new();
    context.insert("ent", &ent);
    // context.insert("flds", &flds);
    context.insert("flds", &ent.fields.iter()
        .filter(|f| !f.is_primary).collect::<Vec<_>>());
    context.insert("keys", &ent.fields.iter()
        .filter(|f| f.is_primary).collect::<Vec<_>>());
    context.insert("multi_pk", &ent.multiple_keys);
    let result = generator.tera.render(template, &context)?;

    Ok(result)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn proto_gen_works() -> anyhow::Result<()> {
        let code=generate_for_proto("proto", "PartyRole")?;
        println!("{}", code);

        let code=generate_for_proto("proto", "Payment")?;
        println!("{}", code);
        Ok(())
    }

}

