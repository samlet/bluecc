#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use structopt::StructOpt;
use meta_gen::{SrvDeles, ServiceMeta};
use seed::{FIELD_MAPPINGS};
use inflector::Inflector;

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

            println!("all params ->");
            for f in params {
                let mut qtype = f.type_name.to_owned();
                if !f.type_name.is_title_case() {
                    qtype = FIELD_MAPPINGS.query_type(f.type_name.as_str());
                }
                println!("\t {}: {}/{} ({:?},{})", f.name,
                         f.type_name, qtype, f.mode,
                         if f.optional { "optional" } else { "required" });
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

