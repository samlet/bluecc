use seed::{EntityGenerator, ModelField, FIELD_MAPPINGS};
use std::collections::HashMap;
use tera::Context;
use serde_json::Value;
use inflector::Inflector;

fn eth_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let val:String = FIELD_MAPPINGS.eth_type(value.as_str().unwrap());
    Ok(Value::String(format!("{}", val)))
}

fn var_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
            let val = value.as_str().unwrap().to_camel_case();
            Ok(Value::String(format!("{}", val)))
        }

pub fn generate_for_eth(template: &str, ent_name: &str) -> crate::Result<String> {
    let ent = seed::get_entity_model(ent_name)?;
    let flds = &ent.fields;

    let mut generator = EntityGenerator::new(vec![ent_name.to_string()]);
    generator.tera.register_filter("eth_type", eth_type);
    generator.tera.register_filter("var_name", var_name);
    generator.tera.add_raw_template("eth", include_str!("incls/eth_obj.j2"))?;

    let mut context = tera::Context::new();
    context.insert("ent", &ent);
    context.insert("flds", flds);
    let result = generator.tera.render(template, &context)?;

    Ok(result)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn eth_obj_gen_works() -> anyhow::Result<()> {
        let code=generate_for_eth("eth", "PartyRole")?;
        println!("{}", code);

        let code=generate_for_eth("eth", "Product")?;
        println!("{}", code);
        Ok(())
    }
}

