#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;

// use std::env;
use structopt::StructOpt;
use std::collections::HashMap;
use entity_seed::meta::app_context::{APP_CONTEXT, FIELD_MAPPINGS, get_entities_by_module_names};
use std::path::PathBuf;
use entity_seed::meta_model::{ModelField, EntityModel, BelongsTo};
use entity_seed::meta::seed_conf::SeedConfig;
use entity_seed::snowflake::new_snowflake_id;
use entity_seed::{GenericError, get_entity_model, get_entity_module};
use tera::{Context, Tera};
use entity_seed::meta::resource_loader::list_data_files;
use entity_seed::meta::*;

/**
```bash
$ cargo run --bin seed gen Example ent
$ cargo run --bin seed gen ExampleStatus dto
$ cargo run --bin seed gen UserLogin dto
$ cargo run --bin seed list security
$ cargo run --bin seed all security
$ cargo run --bin seed wrapper security party
# $ cargo run --bin seed -- -o out_file list
$ bluecc model-files  # 合并压缩所有的模型定义和数据文件
$ bluecc entity StatusItem
$ bluecc entity all
$ bluecc seed Person
$ bluecc list-services | grep Person
```
*/

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Generate specific entity with the type name
    Gen { entity: String, type_name:String },
    /// Generate specific modules: sql-files, model, dto
    All {
        #[structopt(short)]
        only_dto: bool,
        modules: Vec<String>
    },
    /// List all entities in a module
    List { module: String},
    /// Generate model-types
    Wrapper {modules: Vec<String>} ,
    /// List all data files and it's entities
    ListDataFiles,
    /// Build all seed data files into a compressed resource file
    DataFiles,
    /// Build all entity schema into a compressed resource file
    ModelFiles,
    /// Build all service schema into a compressed resource file
    ServiceFiles,
    /// Show entity meta-info
    Entity {name: String},
    /// Show entity seed data
    Seed {name: String},
    /// Show service meta-info
    Service {name: String},
    /// List all service names
    ListServices,
}

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {

    // use tempfile::Builder as TempfileBuilder;
    // let tempfile = TempfileBuilder::new().tempfile_in("./")?;
    //
    // println!(".. generate files .. to {}", args.output
    //     .or(Some(PathBuf::from(tempfile.path())))
    //     .unwrap().display());
    println!(".. execute => {:?}", Command::from_args());

    std::env::set_var("RUST_LOG", "info,entity_seed=info,seed=debug");
    env_logger::init();

    let dir=&cc_conf()?.ofbiz_loc;
    match args.cmd {
        Some(Command::Gen { entity, type_name }) => {
            let generator=EntityGenerator::new(vec![entity.to_owned()]);
            generator.entity_gen_works(entity.as_str(), type_name.as_str())
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
        Some(Command::List { module  }) => {
            let model=get_entity_module(module.as_str())?;
            println!("list all entities");
            for ent in &model.entities {
                println!("{}", ent.entity_name);
            }
        }
        Some(Command::All { only_dto, modules }) => {
            generate_models(only_dto, &modules)?;
        }

        Some(Command::Wrapper { modules  }) => {
            let conf=SeedConfig::load()?;

            let mut context = Context::new();
            // let mods=vec!["security".into()];
            let ents=get_entities_by_module_names(&modules);
            // let names=ents.iter().map(|e|e.entity_name).collect();
            context.insert("ents", &ents);
            context.insert("modules", &modules);
            let result=Tera::one_off(include_str!("incls/seed_wrapper.j2"), &context, true)?;
            println!("{}", result);
            std::fs::write(conf.seed_wrapper, result)?;
        }

        Some(Command::ListDataFiles {  }) => {
            list_data_files()?;
        }
        Some(Command::ModelFiles {  }) => {
            let zout=merge_files("./entitydef", "**/*.xml",
                "./.store/entity_model_files.json", &FileTypes::EntityModel)?;
            println!("save entity models to {}", zout);
        }
        Some(Command::DataFiles {  }) => {
            let zout = merge_files(dir, "**/data/*.xml",
                                   "./.store/seed_files.json", &FileTypes::Data)?;
            println!("save seeds to {}", zout);
        }
        Some(Command::ServiceFiles {  }) => {
            let zout=merge_files(dir, "**/servicedef/*.xml",
                "./.store/service_model_files.json", &FileTypes::ServiceModel)?;
            println!("save service models to {}", zout);
        }

        Some(Command::Entity { name  }) => {
            let mut reader=ModelReader::load()?;
            if name=="all"{
                let items=reader.get_all_entity_names();
                println!("total {} => {:?}", items.len(), items);
            }else {
                let ent = reader.get_entity_model(name.as_str());
                match ent {
                    Ok(ent) => {
                        let ent_json = serde_json::to_string_pretty(&ent)?;
                        println!("{}", ent_json);
                    }
                    _ => ()
                }
            }

        }

        Some(Command::Seed { name  }) => {
            load_seed_model_z_file(name.as_str(), |n| {
                println!("{} ({:?})", n.tag_name().name(), n.range());
                for attr in n.attributes() {
                    println!("\t{} = {}", attr.name(), attr.value());
                }
                true
            })?;
        }

        Some(Command::Service { name  }) => {
            let mut sr = ServiceModelReader::new()?;
            let item = sr.get_service_model(name.as_str())?;
            let json_str = serde_json::to_string_pretty(&item)?;
            println!("{} => {}", name, json_str);
        }

        Some(Command::ListServices {   }) => {
            let sr = ServiceModelReader::new()?;

            for (i, srv) in sr.get_all_service_names().iter().enumerate(){
                println!("{}. {}", i, srv);
            }
        }

        None => {
            println!(".. specific a subcommand.");
            // entity_gen_works("Example", "ent")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
            // entity_gen_works("Example", "model")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
            // entity_gen_works("Example", "dto")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
            //
            // println!(".. model ExampleItem");
            // entity_gen_works("ExampleItem", "ent")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
            // entity_gen_works("ExampleItem", "model")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
            // entity_gen_works("ExampleItem", "dto")
            //     .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
    }

    Ok(())
}

fn generate_models(only_dto: bool, modules: &Vec<String>) -> Result<(), GenericError>{
    let conf=SeedConfig::load()?;

    for module in modules {
        let mut context = Context::new();
        context.insert("module", &module);

        let model = get_entity_module(module.as_str())?;
        let generator=EntityGenerator::new(model.entity_names());
        let module_conf = &conf.module_conf(module.as_str()).expect("absent module conf");
        let model_file = module_conf.model.to_owned();
        let up_sql_file = module_conf.up_sql.to_owned();
        let down_sql_file = module_conf.down_sql.to_owned();

        info!("generate all entities to {} ..", model_file);

        let header = &conf.get_header(module.as_str());
        let enum_header = &conf.get_enum_header(module.as_str());
        let enum_footer = &conf.enum_footer.to_owned();
        // let enum_output=&conf.enum_output.unwrap().to_owned();
        let seed_output = &conf.seed_types.to_owned();

        let gen = move |typs: Vec<&str>, write_header: bool| {
            let mut output = String::new();
            if write_header {
                output.push_str(header.as_str());
            }
            for typ in typs {
                debug!(".. create from {} entities -> {}",model.entities.len(), typ);

                let mut ents: Vec<String> = model.entities.iter()
                    .map(|x| x.entity_name.clone()).collect::<Vec<String>>();
                if typ == "ent_drop" {
                    // ents.reverse();
                    debug!(".. model.topo");
                    ents = model.topo();
                } else if typ == "enum" {
                    output.push_str(enum_header.as_str());
                }
                for ent in &ents {
                    println!("generate {} for {}", ent, typ);

                    let cnt: String = generator.entity_gen_works(ent.as_str(), typ).unwrap();
                    output.push_str(cnt.as_str());
                }

                if typ == "enum" {
                    output.push_str(enum_footer);
                }
            }
            output
        };

        if !only_dto {
            // model relates ...
            std::fs::write(model_file, gen(vec!["model"], true))?;
            // std::fs::write(enum_output, gen(vec!["enum"], false))?;
            std::fs::write(Tera::one_off(seed_output, &context, true)?,
                           gen(vec!["dto_seed"], true))?;

            // sql relates ...
            std::fs::write(up_sql_file, gen(vec!["ent", "ent_rel"], false))?;
            info!("generate drop script ...");
            std::fs::write(down_sql_file, gen(vec!["ent_drop"], false))?;

        }else{
            std::fs::write(Tera::one_off(seed_output, &context, true)?,
                           gen(vec!["dto_seed"], true))?;
        }
        println!("done.");
    }

    Ok(())
}

struct EntityGenerator{
    entities: Vec<String>,
    pub belongs_filter: bool,
}
impl EntityGenerator {
    pub fn new(entities: Vec<String>) -> Self {
        EntityGenerator { entities, belongs_filter: true }
    }

    fn entity_gen_works(&self, entity_name: &str, template_name: &str)
                        -> Result<String, GenericError> {
        use tera::{Result, Context, Filter, Function};
        use tera::Tera;
        use serde_json::{json, Value};

        struct SqlType;
        impl Filter for SqlType {
            fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
                let val = FIELD_MAPPINGS.sql_type(value.as_str().unwrap());
                Ok(Value::String(format!("{}", val)))
            }

            fn is_safe(&self) -> bool {
                true
            }
        }
        fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = inflector::cases::snakecase::to_snake_case(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }
        fn query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.query_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }

        fn orig_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.orig_type(value.as_str().unwrap());
            if val.starts_with("Option") {
                Ok(Value::String(format!("{}", val)))
            } else {
                Ok(Value::String(format!("Option<{}>", val)))
            }
        }

        fn opt_query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.query_type(value.as_str().unwrap());
            if val.starts_with("Option") {
                Ok(Value::String(format!("{}", val)))
            } else {
                Ok(Value::String(format!("Option<{}>", val)))
            }
        }
        fn insert_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.insert_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }
        fn fk_name(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            if value.as_str().unwrap() == "" {
                Ok(Value::String(format!("fk_{}", new_snowflake_id())))
            } else {
                Ok(value.clone())
            }
        }

        // let model=&APP_CONTEXT.get_model(module);
        let ent = get_entity_model(entity_name)?;
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
        tera.add_raw_template("dto_seed", include_str!("incls/dto_seed.j2"))?;
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

        let belongs = &ent.belongs();
        if !self.belongs_filter {
            context.insert("belongs", belongs);
            let has_rels = belongs.len() > 0;
            context.insert("has_rels", &has_rels);
        }else{
            let belongs=belongs.iter()
                .filter(|e|self.entities.contains(&e.model_name))
                .collect::<Vec<&BelongsTo>>();
            context.insert("belongs", &belongs);
            let has_rels = belongs.len() > 0;
            context.insert("has_rels", &has_rels);
        }

        let result = tera.render(template_name, &context)?;
        Ok(result)
    }

}
