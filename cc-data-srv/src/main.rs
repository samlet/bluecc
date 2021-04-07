#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_mut))]

#[macro_use] extern crate log;
#[macro_use]
extern crate serde_derive;

mod handlers;
mod common;
mod directors;
mod auth;
mod acl;
mod dummy;

use std::env;
use structopt::StructOpt;
use crate::handlers::{api_filters};
use deles::delegators::Delegator;
use crate::common::handle_rejection;
use crate::auth::handlers::login_routes;
use std::sync::Arc;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Dummy,
    Login,
    Done { id: u64 },
}

/**
```bash
$ cargo run -- login
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
        Some(Command::Dummy) => {
            let enforcer = Arc::new(Enforcer::new(MODEL_PATH, POLICY_PATH).await
                .expect("can read casbin model and policy files"));
            println!(".. dummy listening on 8080 ..");
            let routes=dummy::handlers::dummy_routes(enforcer);
            warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
        }
        Some(Command::Done { id }) => {
            println!(".. {} done", id);
        }
        Some(Command::Login) => {
            println!(".. srv listening on 8000 ..");
            let routes=login_routes();
            warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
        }

        None => {
            println!(".. srv listening on 3030 ..");
            let delegator = Delegator::new().await?;
            // let api = api_filters(delegator);
            let api = api_filters(delegator)
                .recover(handle_rejection);
            // View access logs by setting `RUST_LOG=todos`.
            let routes = api.with(warp::log("bluecc"));

            warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
        }
    }

    Ok(())
}

