use seed::{EntityGenerator, ModelField, FIELD_MAPPINGS};
use std::collections::HashMap;
use tera::Context;
use serde_json::Value;
use inflector::Inflector;
use itertools::Itertools;
use crate::{GenericError, ServiceMeta, ParamMode, ModelParam};

fn java_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let val:String = FIELD_MAPPINGS.java_type(value.as_str().unwrap());
    Ok(Value::String(format!("{}", val)))
}

pub fn generate_for_entity(template: &str, ent_name: &str) -> crate::Result<String> {
    let ent:seed::Entity = seed::get_entity_model(ent_name)?;
    // let flds = &ent.fields;

    let mut generator = EntityGenerator::new(vec![ent_name.to_string()]);
    generator.tera.register_filter("java_type", java_type);
    generator.tera.add_raw_template("java_obj", include_str!("incls/java_obj.j2"))?;

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


pub fn generate_for_service(writer: &mut dyn std::io::Write,
                            srv_meta: &mut ServiceMeta, srv_name: &str)
    -> Result<(), GenericError> {
    let srv=srv_meta.srv(srv_name)?.to_owned();
    let params = srv_meta.srv_params(srv_name)?;

    for p in &params {
        debug!("{} {:?} {}", p.name, p.mode, p.optional);
    }

    let inputs = params.iter()
        .filter(|p| p.mode == ParamMode::In || p.mode == ParamMode::InOut)
        .collect::<Vec<&ModelParam>>();
    let outputs = params.iter()
        .filter(|p| p.mode == ParamMode::Out || p.mode == ParamMode::InOut)
        .collect::<Vec<&ModelParam>>();
    debug!("outputs: {}", outputs.len());

    // generate
    let mut generator = EntityGenerator::new(vec![srv.get_entity_name()]);
    generator.tera.register_filter("java_type", java_type);
    generator.tera.add_raw_template("java_req", include_str!("incls/java_req.j2"))?;
    generator.tera.add_raw_template("java_resp", include_str!("incls/java_resp.j2"))?;

    let mut context = Context::new();
    context.insert("srv", &srv);
    context.insert("inputs", &inputs);
    context.insert("opts", &inputs.iter()
        .filter(|&p| p.optional)
        .collect::<Vec<&&ModelParam>>()
    );
    let reqs = inputs.iter()
        .filter(|&p| !p.optional)
        .map(|p| format!("{} {}",
                         p.param_type(), p.name))
        .collect::<Vec<String>>();
    let reqs_str = reqs.join(", ");
    context.insert("reqs", &reqs_str);
    context.insert("outputs", &outputs);

    let result = generator.tera.render("java_req", &context)?;
    writeln!(writer, "{}", result)?;

    if !outputs.is_empty() {
        let result = generator.tera.render("java_resp", &context)?;
        // println!("result => \n{}", result);
        writeln!(writer, "{}", result)?;
    }

    Ok(())
}


#[cfg(test)]
mod lib_tests {
    use super::*;
    use thiserror::private::PathAsDisplay;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn java_obj_gen_works() -> anyhow::Result<()> {
        let code=generate_for_entity("java_obj", "PartyRole")?;
        println!("{}", code);

        let code=generate_for_entity("java_obj", "Payment")?;
        println!("{}", code);
        Ok(())
    }

    #[test]
    fn generate_for_service_works() -> anyhow::Result<()> {
        let mut srvmeta=ServiceMeta::load()?;
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        generate_for_service(&mut handle, &mut srvmeta, "createPerson")?;
        generate_for_service(&mut handle, &mut srvmeta, "testScv")?;
        Ok(())
    }

    #[test]
    fn generate_ent_works() -> anyhow::Result<()> {
        let entity_name="Person";

        let ent_path = "src/main/java/com/bluecc/api/models";
        // let srv_path = "src/main/java/com/bluecc/api/activities";

        let ent = seed::get_entity_model(entity_name)?;
        let pkg = ent.package_name;
        let sub_path = pkg.trim_start_matches("org.apache.ofbiz.").split(".").join("/");
        let target_dir = dirs::home_dir().unwrap();
        let target_file = target_dir.join("dispat").join(ent_path).join(sub_path)
            .join(format!("{}.java", entity_name));
        let sub_dir = target_file.parent().unwrap();
        if !sub_dir.exists() {
            std::fs::create_dir_all(sub_dir)?;
        }

        println!("write to {}.", target_file.as_display());
        let code=generate_for_entity("java_obj", entity_name)?;
        let mut file = File::create(target_file)?;
        let pkg_def=format!("package com.bluecc.api.models.{};\n\n",
                            pkg.trim_start_matches("org.apache.ofbiz."));
        file.write_all(pkg_def.as_bytes())?;
        file.write_all(code.as_bytes())?;
        Ok(())
    }
}

