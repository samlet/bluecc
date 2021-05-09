#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

use std::env;
use structopt::StructOpt;
use std::sync::Arc;
use blueprints::handlers;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Done { id: u64 },
}

/**
```bash
$ cargo run
$ curl localhost:3030/hi
```
*/

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    use casbin::prelude::*;
    use warp::Filter;

    const MODEL_PATH: &str = "./dummy/auth_model.conf";
    const POLICY_PATH: &str = "./dummy/policy.csv";

    std::env::set_var("RUST_LOG", "info,entity_seed=info,meta_gen=info");
    env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Done { id }) => {
            println!(".. {} done", id);
        }
        None => {
            let enforcer = Arc::new(Enforcer::new(MODEL_PATH, POLICY_PATH).await
                .expect("can read casbin model and policy files"));
            println!(".. dummy listening on 9099 ..");
            let routes=handlers::dummy_routes(enforcer);
            warp::serve(routes).run(([0, 0, 0, 0], 9099)).await;
        }
    }

    Ok(())
}

