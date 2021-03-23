#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod generator;

use std::env;
use structopt::StructOpt;
use meta_gen::{SrvDeles, ServiceMeta, ParamMode, ModelParam,
               GenericError, DynamicValue, SrvResp};
use seed::{FIELD_MAPPINGS};
use seed::meta::{load_seed_model_z_file, ModelService, SeedFiles};
use inflector::Inflector;
use colored::*;
use std::collections::HashMap;
use serde_json::Value;
use roxmltree::Node;
use crate::generator::MetaGenerator;

#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;

/*
$ cargo run -- srv createExample
$ meta-cli srv -c createPerson
$ meta-cli srv -c -e updatePerson
$ cargo run -- call testScv
$ meta-cli seed Person plain
$ meta-cli seed Person json-init
$ meta-cli entity Person ink
$ meta-cli dump spec-srv > .store/spec-srvs.txt
 */

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Show service parameters
    Srv {
        #[structopt(short)]
        collapse: bool,
        #[structopt(short)]
        example: bool,
        name: String
    },
    Entity {
        name: String,
        template: String,
    },
    /// Call service
    Call { name: String},
    /// Find entity records
    Find { entity_name: String},
    /// Show entity seed-data
    Seed {
        entity_name: String,
        #[structopt(default_value = "plain")]
        format: String
    },
    /// Get the default access token
    Token,
    Dump {spec: String},
}

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,entity_seed=info,meta_gen=info");
    env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Srv { collapse, example, name }) => {
            println!("srv-meta {}", name);
            let mut srvs = ServiceMeta::load()?;
            let srv= srvs.srv(name.as_str())?.to_owned();
            let srv_ent=&srv.default_entity_name;
            let srv_ent_incs=srv.include_auto_attrs();
            let params = srvs.srv_params(name.as_str())?;

            println!("input params ->");
            if collapse && !srv_ent.is_empty(){
                println!("\t default entity {} ({})", srv_ent.red().bold(), srv_ent_incs.yellow());
            }
            for f in params.iter().filter(|p|p.mode==ParamMode::In || p.mode==ParamMode::InOut) {
                let mut ptype=f.param_type();
                if let Some(v)=&f.entity_name{
                    ptype=format!("{}.{}", v.cyan(), ptype);
                    if collapse && !f.overload{
                        continue; // skip the parameter if collapse
                    }
                }
                println!("\t {}: {}/{} ({:?},{})", f.name.black().bold(),
                         f.type_name, ptype, f.mode,
                         if f.optional { "optional".yellow() } else { "required".blue().bold() });
            }

            println!("output params ->");
            for f in params.iter().filter(|p|p.mode==ParamMode::Out || p.mode==ParamMode::InOut) {
                println!("\t {}: {}/{} ({:?},{})", f.name.black().bold(),
                         f.type_name, f.param_type(), f.mode,
                         if f.optional { "optional".yellow() } else { "required".blue().bold() });
            }

            if example{
                println!("example ->");
                output_invoke_example(&srv)?;
            }
        }

        Some(Command::Call { name }) => {
            invoke_srv(name.as_str()).await?;
        }

        Some(Command::Token {  }) => {
            let mut dele=SrvDeles::new();
            dele.use_default_token().await?;
            println!("tok -> {}", dele.access_token);
        }

        Some(Command::Find { entity_name }) => {
            use deles::delegators::{result_str, Delegator};
            let delegator=Delegator::new().await?;
            let result=delegator.find_all(entity_name.as_str(), false,false).await?;
            let cols = result.rs.columns();
            println!("cols (total {}) {:?}", cols.len(), cols);
            println!("{}", result_str(result).await);
        }

        Some(Command::Seed { entity_name, format  }) => {
            use seed::meta::ModelReader;
            let mut entity_reader= ModelReader::load()?;
            let entity=entity_reader.get_entity_model(entity_name.as_str())?;
            let var_name=entity_name.to_snake_case();

            load_seed_model_z_file(entity_name.as_str(), |n| {
                match format.as_str() {
                    "toml" => {
                        println!("{} ({:?})", n.tag_name().name(), n.range());
                        for attr in n.attributes() {
                            let attr_val=format!("\"{}\"", attr.value());
                            println!("\t{} = {}", attr.name().to_snake_case(), attr_val);
                        }
                    }
                    "json-init" => {
                        println!("let {}_json = json!({{", var_name);
                        json_init_attrs(n, &entity);
                        println!("}});")
                    }
                    "insert" => {

                    }
                    "update" => {

                    }
                    _ =>{
                        println!("{} ({:?})", n.tag_name().name(), n.range());
                        for attr in n.attributes() {
                            println!("\t{} = {}", attr.name(), attr.value());
                        }
                    }
                }

                true
            })?;

            if format=="json-init".to_string() {
                println!("// to initialize =>");
                println!("let {}:{}=serde_json::from_value({}_json)?;", var_name.blue(),
                         entity_name.cyan(), var_name);
            }
        }

        Some(Command::Entity { name, template  }) => {
            let mut srvs = ServiceMeta::load()?;
            let result=srvs.generate_for(template.as_str(), name.as_str())?;
            println!("{}", result);
        }

        Some(Command::Dump { spec  }) => {
            match spec.as_str() {
                "spec-srv" => {
                    let mut srvs = ServiceMeta::load()?;
                    let mut skip_srvs=Vec::new();
                    // notice: take about 4s
                    let mut total=0;
                    for srv_name in srvs.service_reader.get_all_service_names() {
                        let params = srvs.srv_params(srv_name.as_str()).unwrap_or(Default::default());
                        if params.is_empty(){
                            skip_srvs.push(srv_name.to_owned());
                        }
                        let spec_flds = params.iter().filter(|f| f.type_name == "List" || f.type_name == "Map")
                            .map(|f| (&f.name, &f.type_name)).collect::<Vec<(&String, &String)>>();
                        if !spec_flds.is_empty() {
                            println!("{} spec flds: {:?}", srv_name, spec_flds);
                            total+=1;
                        }
                    }
                    println!("total spec-srvs: {}", total);
                    let skips=serde_json::to_string_pretty(&skip_srvs)?;
                    println!("skip srvs: {}", skips);
                }
                _ => ()
            }
        }

        None => {
            println!(".. execute => {:?}", Command::from_args());
        }
    }

    Ok(())
}

fn output_invoke_example(srv:&ModelService) -> Result<(), GenericError>{
    let ent_name = &srv.default_entity_name;
    if !ent_name.is_empty() {
        let seeds = SeedFiles::load()?;

        let rs = seeds.entity_seeds(ent_name)?;
        let mut stats = HashMap::new();
        for r in &rs {
            let fld_num = r.len();
            let entry = stats.entry(fld_num).or_insert(1);
            *entry += 1;
        }

        // 找到最常用的字段组合(即这个组合的频次最高)
        let max_item = stats.iter()
            .max_by(|f, s| f.1.cmp(s.1)).unwrap();
        // println!("{:?} => {:?}", max_item, stats);
        let exflds = rs.iter().filter(|&r| r.len() == *max_item.0)
            .nth(0).unwrap();
        let exflds_str = serde_json::to_string_pretty(&exflds)?;
        println!("let p: {} = serde_json::from_value(json!({}))?;", ent_name, exflds_str);
    }

    Ok(())
}

fn quaint_insert_attrs(n:&Node, ent:&seed::Entity) {
    for attr in n.attributes() {
        let fld=ent.get_field(attr.name()).expect("fld");
        let _fld_type=FIELD_MAPPINGS.quaint_type(fld.field_type.as_str());
    }
}

fn json_init_attrs(n:&Node, ent:&seed::Entity) {
    for attr in n.attributes() {
        let fld=ent.get_field(attr.name()).expect("fld");
        match fld.field_type.as_str() {
            "currency-amount" | "currency-precise" | "numeric" |
            "fixed-point" | "floating-point" | "integer"
            => {
                println!("\t{} = {},", attr.name().to_snake_case(), attr.value());
            }
            "date-time" => {
                println!("\t{} = NaiveDateTime::parse_from_str(\"{}\", \"%Y-%m-%d %H:%M:%S%.f\"),",
                    attr.name().to_snake_case(), attr.value());
            }
            _ => {
                let attr_val = format!("\"{}\"", attr.value());
                println!("\t{} = {},", attr.name().to_snake_case(), attr_val);
            }
        }

    }
}

async fn invoke_srv(srv_name: &str) -> Result<(), GenericError> {
    use console::Style;
    use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let mut srvs = ServiceMeta::load()?;
    let params = srvs.srv_params(srv_name)?;

    let mut ctx=DynamicValue{ values: HashMap::new() };

    println!("execute service {},  input params ->", srv_name);
    for f in params.iter().filter(|p| p.mode == ParamMode::In || p.mode == ParamMode::InOut) {
        let mut ptype = f.param_type();
        if let Some(v) = &f.entity_name {
            ptype = format!("{}.{}", v.cyan(), ptype);
        }
        println!("\t {}: {}/{} ({:?},{})", f.name.black().bold(),
                 f.type_name, ptype, f.mode,
                 if f.optional { "optional".yellow() } else { "required".blue().bold() });

        let val = Input::with_theme(&theme)
            .with_prompt(&f.name)
            .default("".to_string())
            .interact()?;
        ctx.values.insert(f.name.to_owned(), Value::from(val));
    }

    srv_invoke_with_dynamic_works(srv_name, &ctx).await?;

    Ok(())
}

async fn srv_invoke_with_dynamic_works(srv_name: &str, ctx: &DynamicValue) -> Result<(), GenericError> {
    let mut dele=SrvDeles::new();
    dele.use_default_token().await?;
    println!("tok {}", dele.access_token);
    // values.insert("defaultValue".to_string(), Value::from(8.8));
    // values.insert("message".to_string(), Value::from("hi"));
    let ret: SrvResp<DynamicValue>=dele.srv(srv_name, ctx).await?;
    let data_json=serde_json::to_string_pretty(&ret)?;
    println!("{}", data_json);

    Ok(())
}

