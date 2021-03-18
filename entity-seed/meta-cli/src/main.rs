#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use structopt::StructOpt;
use meta_gen::{SrvDeles, ServiceMeta, ParamMode, ModelParam};
use seed::{FIELD_MAPPINGS};
use inflector::Inflector;
use colored::*;

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
                println!("\t {}: {}/{} ({:?},{})", f.name.black().bold(),
                         f.type_name, f.param_type(), f.mode,
                         if f.optional { "optional".yellow() } else { "required".blue().bold() });
            }
            println!("output params ->");
            for f in params.iter().filter(|p|p.mode==ParamMode::Out || p.mode==ParamMode::InOut) {
                println!("\t {}: {}/{} ({:?},{})", f.name.black().bold(),
                         f.type_name, f.param_type(), f.mode,
                         if f.optional { "optional".yellow() } else { "required".blue().bold() });
            }
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

