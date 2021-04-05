#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

use std::env;
use structopt::StructOpt;
use cc_bridge::srv_director::SrvDirector;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Conf { item: String },
    Srv,
}

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,entity_seed=info,meta_gen=info");
    env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Conf { item }) => {
            println!("Get service conf from {}", item);
        }
        Some(Command::Srv) => {
            // ...
        }
        None => {
            println!(".. srv listening ..");
            let sd=SrvDirector::new();
            sd.srv().await?;
        }
    }

    Ok(())
}

