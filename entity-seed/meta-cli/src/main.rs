#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use structopt::StructOpt;
use meta_gen::{SrvDeles, ServiceMeta, ParamMode, ModelParam, GenericError, DynamicValue, SrvResp};
use seed::{FIELD_MAPPINGS};
use inflector::Inflector;
use colored::*;
use std::collections::HashMap;
use serde_json::Value;

#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;

/*
$ cargo run -- srv createExample
 */

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Show service parameters
    Srv { name: String},
    Call { name: String},
    /// Get the default access token
    Token,
}

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,entity_seed=debug,meta_gen=info");
    env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Srv { name }) => {
            println!("srv-meta {}", name);
            let mut srvs = ServiceMeta::load()?;
            let params = srvs.srv_params(name.as_str())?;

            println!("input params ->");
            for f in params.iter().filter(|p|p.mode==ParamMode::In || p.mode==ParamMode::InOut) {
                let mut ptype=f.param_type();
                if let Some(v)=&f.entity_name{
                    ptype=format!("{}.{}", v.cyan(), ptype);
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
        }

        Some(Command::Call { name }) => {
            invoke_srv(name.as_str()).await?;
        }

        Some(Command::Token {  }) => {
            let mut dele=SrvDeles::new();
            dele.use_default_token().await?;
            println!("tok -> {}", dele.access_token);
        }

        None => {
            println!(".. execute => {:?}", Command::from_args());
        }
    }

    Ok(())
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

