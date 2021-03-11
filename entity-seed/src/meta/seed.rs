#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use]
extern crate lazy_static;

// use std::env;
use structopt::StructOpt;
use std::collections::HashMap;
use entity_seed::meta::app_context::{APP_CONTEXT};
use std::path::PathBuf;
use entity_seed::meta_model::{ModelField, EntityModel};
use entity_seed::meta::seed_conf::SeedConfig;
use entity_seed::snowflake::new_snowflake_id;
use entity_seed::GenericError;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(StructOpt)]
enum Command {
    Gen { module: String, entity: String, type_name:String },
    All { module: String},
    List { module: String},
}

/**
```bash
$ cargo run --bin seed gen Example ent
$ cargo run --bin seed gen example ExampleStatus dto
$ cargo run --bin seed gen security UserLogin dto
$ cargo run --bin seed list security
$ cargo run --bin seed all security
# $ cargo run --bin seed -- -o out_file list
```
*/

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {

    // use tempfile::Builder as TempfileBuilder;
    // let tempfile = TempfileBuilder::new().tempfile_in("./")?;
    //
    // println!(".. generate files .. to {}", args.output
    //     .or(Some(PathBuf::from(tempfile.path())))
    //     .unwrap().display());

    match args.cmd {
        Some(Command::Gen { module, entity, type_name }) => {
            entity_gen_works(module.as_str(), entity.as_str(), type_name.as_str())
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
        Some(Command::List { module  }) => {
            let model=&APP_CONTEXT.get_model(module.as_str());
            println!("list all entities");
            for ent in &model.entities {
                println!("{}", ent.entity_name);
            }
        }
        Some(Command::All { module }) => {
            let model=&APP_CONTEXT.get_model(module.as_str());
            let conf=SeedConfig::load()?;
            let module_conf=&conf.module_conf(module.as_str()).unwrap();
            let model_file=&module_conf.model.to_owned();
            let up_sql_file=module_conf.up_sql.to_owned();
            let down_sql_file=module_conf.down_sql.to_owned();

            println!("generate all entities to {} ..", model_file);

            let header=&conf.get_header(module.as_str());
            let enum_header=&conf.get_enum_header(module.as_str());
            let enum_footer=&conf.enum_footer.unwrap().to_owned();
            let enum_output=&conf.enum_output.unwrap().to_owned();

            let gen = |typs:Vec<&str>, write_header:bool|  {
                let mut output=String::new();
                if write_header {
                    output.push_str(header.as_str());
                }
                for typ in typs {
                    let mut ents:Vec<String>= model.entities.iter()
                        .map(|x|x.entity_name.clone()).collect::<Vec<String>>();
                    if typ=="ent_drop"{
                        // ents.reverse();
                        ents=model.topo();
                    }else if typ=="enum"{
                        output.push_str(enum_header.as_str());
                    }
                    for ent in &ents {
                        println!("generate {} for {}", ent, typ);

                        let cnt: String = entity_gen_works(module.as_str(), ent.as_str(), typ).unwrap();
                        output.push_str(cnt.as_str());
                    }

                    if typ=="enum"{
                        output.push_str(enum_footer);
                    }
                }
                output
            };

            std::fs::write(model_file, gen(vec!["model"], true))?;
            std::fs::write(up_sql_file, gen(vec!["ent", "ent_rel"], false))?;
            std::fs::write(down_sql_file, gen(vec!["ent_drop"], false))?;
            std::fs::write(enum_output, gen(vec!["enum"], false))?;
            println!("done.");
        }

        None => {
            let module="example";
            println!(".. model Example");
            entity_gen_works(module, "Example", "ent")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "Example", "model")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "Example", "dto")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();

            println!(".. model ExampleItem");
            entity_gen_works(module, "ExampleItem", "ent")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "ExampleItem", "model")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "ExampleItem", "dto")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
    }

    Ok(())
}

fn entity_gen_works(module:&str, entity_name: &str, template_name: &str) -> Result<String, GenericError> {
    use tera::{Result, Context, Filter, Function};
    use tera::Tera;
    use serde_json::{json, Value};

    struct SqlType;
    impl Filter for SqlType {
        fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val=APP_CONTEXT.field_mappings.sql_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }

        fn is_safe(&self) -> bool {
            true
        }
    }
    fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=inflector::cases::snakecase::to_snake_case(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }
    fn query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.query_type(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }

    fn orig_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.orig_type(value.as_str().unwrap());
        if val.starts_with("Option") {
            Ok(Value::String(format!("{}", val)))
        }else{
            Ok(Value::String(format!("Option<{}>", val)))
        }
    }

    fn opt_query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.query_type(value.as_str().unwrap());
        if val.starts_with("Option") {
            Ok(Value::String(format!("{}", val)))
        }else{
            Ok(Value::String(format!("Option<{}>", val)))
        }
    }
    fn insert_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.insert_type(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }
    fn fk_name(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        if value.as_str().unwrap()=="" {
            Ok(Value::String(format!("fk_{}", new_snowflake_id())))
        }else{
            Ok(value.clone())
        }
    }

    let model=&APP_CONTEXT.get_model(module);
    let ent=model.get_entity(entity_name);
    assert_eq!(entity_name, ent.entity_name);
    // for f in &ent.fields {
    //     println!("* {}: {}", f.field_name, f.is_primary);
    // }

    let mut tera = Tera::default();
    tera.add_raw_template("ent", include_str!("incls/ent.j2"))?;
    tera.add_raw_template("ent_rel", include_str!("incls/ent_rel.j2"))?;
    tera.add_raw_template("ent_drop", include_str!("incls/ent_drop.j2"))?;
    tera.add_raw_template("model", include_str!("incls/model.j2"))?;
    tera.add_raw_template("dto", include_str!("incls/dto.j2"))?;
    tera.add_raw_template("dto_orig", include_str!("incls/dto_orig.j2"))?;

    /*
    #[serde(rename_all = "camelCase")]
    UserLogin {
        user_login_id: String,
        enabled: Option<String>,
        nick_name: Option<String>,
    },
     */
    tera.add_raw_template("enum", include_str!("incls/enum.j2"))?;

    let mut context = Context::new();
    tera.register_filter("sqltype", SqlType);
    tera.register_filter("query_type", query_type);
    tera.register_filter("opt_query_type", opt_query_type);
    tera.register_filter("orig_type", orig_type);
    tera.register_filter("insert_type", insert_type);
    tera.register_filter("snake_case", snake_case);
    tera.register_filter("fk", fk_name);
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields.iter()
        .filter(|f| !f.is_primary).collect::<Vec<_>>());
    context.insert("keys", &ent.fields.iter()
        .filter(|f| f.is_primary).collect::<Vec<_>>());
    context.insert("multi_pk", &ent.multiple_keys);
    context.insert("pks", &ent.pks_str());
    let belongs=&ent.belongs();
    context.insert("belongs", &belongs);
    let has_rels=belongs.len()>0;
    context.insert("has_rels", &has_rels);

    let result = tera.render(template_name, &context)?;
    Ok(result)
}

