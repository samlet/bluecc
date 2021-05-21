use seed::{EntityGenerator, ModelField, FIELD_MAPPINGS};
use std::collections::HashMap;
use tera::Context;
use serde_json::Value;
use inflector::Inflector;
use itertools::Itertools;

fn capnp_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let val:String = FIELD_MAPPINGS.capnp_type(value.as_str().unwrap());
    Ok(Value::String(format!("{}", val)))
}

pub fn generate_for_capnp(template: &str, ent_name: &str) -> crate::Result<String> {
    let ent:seed::Entity = seed::get_entity_model(ent_name)?;
    let mut generator = EntityGenerator::new(vec![ent_name.to_string()]);
    generator.tera.register_filter("capnp_type", capnp_type);
    generator.tera.add_raw_template("capnp", include_str!("incls/capnp_obj.j2"))?;

    let mut context = tera::Context::new();
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields.iter()
        .filter(|f| !f.is_primary).collect::<Vec<_>>());
    context.insert("keys", &ent.fields.iter()
        .filter(|f| f.is_primary).collect::<Vec<_>>());
    let pks_num=ent.pks().len()-1;
    context.insert("multi_pk", &ent.multiple_keys);
    context.insert("fld_start", &pks_num);
    let result = generator.tera.render(template, &context)?;

    Ok(result)
}

fn gen_capnp_id() -> String {
    use std::process::{Command, Stdio};
    let output = Command::new("capnp")
        .arg("id")
        // Tell the OS to record the command's output
        .stdout(Stdio::piped())
        // execute the command, wait for it to complete, then capture the output
        .output()
        // Blow up if the OS was unable to start the program
        .unwrap();
    // extract the raw bytes that we captured and interpret them as a string
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}

#[derive(Serialize, Deserialize, Debug)]
struct CapnpIntf{
    name: String,
    parent_type: Option<String>,
    services: Vec<CapnpService>,
    modifiers: Vec<CapnpModifier>,
    entry: CapnpEntry,
}

// 包含了default-entity的服务, 使用modifier, 同时生成modifier结构和方法声明
#[derive(Serialize, Deserialize, Debug)]
struct CapnpModifier{
    name: String,
    fields: Vec<CapnpParam>,
    params: Vec<CapnpParam>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CapnpEntry{
    list_fields: Vec<CapnpParam>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CapnpService{
    name: String,
    params: Vec<CapnpParam>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CapnpParam{
    name: String,
    type_name: String,
    default_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CapnpOutline{

}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;

    #[test]
    fn gen_works() -> anyhow::Result<()> {
        println!("{};", gen_capnp_id());
        let code=generate_for_capnp("capnp", "PartyRole")?;
        println!("{}", code);

        let code=generate_for_capnp("capnp", "Payment")?;
        println!("{}", code);
        Ok(())
    }

    fn print_rels(entity_name: &str) -> anyhow::Result<()>{
        // let entity_name="Person";
        let mut srvs = ServiceMeta::load()?;
        let result = srvs.get_related_srvs(entity_name)?;
        let rels = serde_json::to_string_pretty(&result)?;
        println!("{}", rels);

        Ok(())
    }

    #[test]
    fn build_capnp_interface_works() -> anyhow::Result<()> {


        Ok(())
    }
}

