#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use structopt::StructOpt;
use warp::Filter;
use cc_pipes::runner::ScriptEngine;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Script {file: String},
}

/**
```bash
$ cargo run --bin pipes script src/scripts/simple.rhai
$ pipes script src/scripts/simple.rhai
```
*/

#[tokio::main]
#[paw::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let args = Args::from_args();
    match args.cmd {
        Some(Command::Script { file  }) => {
            let mut runner=ScriptEngine::new();
            runner.run_script(file.as_str())?;
        }
        None => {

        }
    }

    Ok(())
}

