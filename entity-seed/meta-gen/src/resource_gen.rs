use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::{GenericError, ParamMode, ServiceMeta, ModelParam};
use seed::{EntityGenerator, Entity, ModelField, FIELD_MAPPINGS};
use seed::meta::{ModelService, ServiceModel, CcConfig, CC_CONF};
use std::collections::HashMap;
use inflector::Inflector;
use tera::Context;

trait ResourceGenerator{
    fn write_to(buffer: &dyn Write) -> Result<(), GenericError>;
}

fn srv_param_type(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let mut type_name:String = value.as_str().unwrap().to_string();
    if !type_name.is_pascal_case() {
        type_name = FIELD_MAPPINGS.orig_type(type_name.as_str());
    }
    Ok(tera::Value::String(type_name))
}

fn cstr_type(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let mut type_name:String = value.as_str().unwrap().to_string();
    if type_name=="String" {
        type_name="&'a str".to_string();
    }
    Ok(tera::Value::String(type_name.to_owned()))
}

fn guess_action(value: &tera::Value, _args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let mut type_name:String = value.as_str().unwrap().to_string();
    let mut action=type_name.to_pascal_case();
    if type_name.starts_with("create") {
        action="Create".to_string();
    }else if type_name.starts_with("update") {
        action="Update".to_string();
    }else if type_name.starts_with("delete") || type_name.starts_with("remove") {
        action="Delete".to_string();
    }
    Ok(tera::Value::String(action))
}

pub fn generate_srv_invoker(writer: &mut dyn std::io::Write, srv: &ModelService,
                            ents: &HashMap<String, Entity>, spec: &str)
    -> Result<(), GenericError> {
    debug!("srv name {} with {}, ents: {:?}", srv.name, srv.get_entity_name(), ents.keys());
    let params = ServiceMeta::srv_model_params(&srv, &ents)?;

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
    let mut generator = EntityGenerator::new(ents.keys().cloned().collect());
    // generator.tera.register_filter("param_type", srv_param_type);
    generator.tera.register_filter("action", guess_action);
    generator.tera.register_filter("cstr", cstr_type);
    generator.tera.add_raw_template("srv_create", include_str!("incls/srv_create.j2"))?;
    generator.tera.add_raw_template("srv_create_cr", include_str!("incls/srv_create_cr.j2"))?;
    generator.tera.add_raw_template("srv_resp", include_str!("incls/srv_resp.j2"))?;
    generator.tera.add_raw_template("srv_req", include_str!("incls/srv_req.j2"))?;
    generator.tera.add_raw_template("srv_req_cr", include_str!("incls/srv_req_cr.j2"))?;

    let mut context = Context::new();
    context.insert("srv", &srv);
    let default_entity_name=srv.get_entity_name();
    if !default_entity_name.is_empty() {
        context.insert("ent", ents.get(default_entity_name.as_str()).unwrap());
    }
    context.insert("inputs", &inputs);
    context.insert("opts", &inputs.iter()
        .filter(|&p| p.optional)
        .collect::<Vec<&&ModelParam>>()
    );
    let reqs = inputs.iter()
        .filter(|&p| !p.optional)
        .map(|p| format!("{}: {}",
                         p.name.to_snake_case(),
                         p.param_value_type(spec)))
        .collect::<Vec<String>>();
    let reqs_str = reqs.join(", ");
    context.insert("reqs", &reqs_str);
    context.insert("outputs", &outputs);

    if !default_entity_name.is_empty() {
        let tpl:String=if spec.is_empty() {"srv_create".to_string()} else {format!("srv_create_{}", spec)};
        let result = generator.tera.render(tpl.as_str(), &context)?;
        // println!("result => \n{}", result);
        writeln!(writer, "{}", result)?;
    }else{
        let tpl=if spec.is_empty() {"srv_req".to_string()} else {format!("srv_req_{}", spec)};
        let result = generator.tera.render(tpl.as_str(), &context)?;
        writeln!(writer, "{}", result)?;
    }

    if !outputs.is_empty() {
        let result = generator.tera.render("srv_resp", &context)?;
        // println!("result => \n{}", result);
        writeln!(writer, "{}", result)?;
    }

    Ok(())
}

pub fn generate_srv_ent(writer: &mut dyn std::io::Write, entity:&Entity) -> Result<(), GenericError> {
    let ent=&entity.entity_name;
    let mut generator = EntityGenerator::new(vec![ent.to_string()]);
    generator.tera.add_raw_template("srv_ent", include_str!("incls/srv_ent.j2"))?;
    let result=generator.entity_gen_with(entity, "srv_ent")?;
    writeln!(writer, "{}", result)?;
    Ok(())
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use crate::get_srv;

    const PROJ_ROOT: &'static str = "../../deles";
    const RESOURCE_ROOT: &'static str = "../../deles/src/resources";

    #[test]
    fn write_tmp_works() -> anyhow::Result<()> {
        let path = Path::new(PROJ_ROOT);
        let mut buffer = File::create(path.join(".store/foo.txt"))?;
        writeln!(buffer, "just a test")?;
        Ok(())
    }

    #[test]
    fn srv_ent_works() -> anyhow::Result<()> {
        let ent="Example";
        let (_,ents)=get_srv("plugins/example", "createExample")?;
        let entity=ents.get(ent).unwrap();
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        generate_srv_ent(&mut handle, &entity)?;
        Ok(())
    }

    #[test]
    fn srv_gen_works() -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let (srv,ents)=get_srv("plugins/example", "createExample")?;
        generate_srv_invoker(&mut handle, &srv, &ents, "")?;
        Ok(())
    }
}


