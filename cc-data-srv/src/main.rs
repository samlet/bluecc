#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate lazy_static;

mod handlers;

use std::env;
use structopt::StructOpt;
use warp::Filter;
use crate::handlers::party;
use deles::delegators::Delegator;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { description: String },
    Srv,
    Done { id: u64 },
}

/**
```bash
$ cargo run -- add hello
$ cargo run
$ curl localhost:3030/hi
```
*/

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,entity_seed=info,meta_gen=info");
    env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{}'", &description);
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {} as done", id);
        }
        Some(Command::Srv) => {
            // ...
        }
        None => {
            println!(".. srv listening on 3030 ..");
            let delegator = Delegator::new().await?;
            let api = party(delegator);
            // View access logs by setting `RUST_LOG=todos`.
            let routes = api.with(warp::log("party"));

            warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
        }
    }

    Ok(())
}

