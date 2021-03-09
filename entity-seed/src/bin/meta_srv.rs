#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use structopt::StructOpt;
use warp::Filter;

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
$ cargo run --bin meta-srv add hello
$ cargo run --bin meta-srv
$ curl localhost:3030/hi
```
*/

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

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

            // GET /
            let hello_world = warp::path::end().map(|| "Hello, World at root!");
            // GET /hi
            let hi = warp::path("hi").map(|| "Hello, World!");
            let routes = warp::get().and(
                hello_world
                    .or(hi),
            );

            // Note that composing filters for many routes may increase compile times (because it uses a lot of generics).
            // If you wish to use dynamic dispatch instead and speed up compile times while
            // making it slightly slower at runtime, you can use Filter::boxed().

            warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
        }
    }

    Ok(())
}

