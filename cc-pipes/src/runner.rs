use rhai::{Engine, EvalAltResult, Position, RegisterFn, INT, Scope};

#[cfg(not(feature = "no_optimize"))]
use rhai::OptimizationLevel;

use std::{env, fs::File, io::Read, process::exit};
use crate::GenericError;

fn eprint_error(input: &str, mut err: EvalAltResult) {
    fn eprint_line(lines: &[&str], pos: Position, err_msg: &str) {
        let line = pos.line().unwrap();
        let line_no = format!("{}: ", line);

        eprintln!("{}{}", line_no, lines[line - 1]);
        eprintln!(
            "{:>1$} {2}",
            "^",
            line_no.len() + pos.position().unwrap(),
            err_msg
        );
        eprintln!("");
    }

    let lines: Vec<_> = input.split('\n').collect();

    // Print error
    // let pos = err.take_position();
    let pos = err.position();
    err.clear_position();

    if pos.is_none() {
        // No position
        eprintln!("{}", err);
    } else {
        // Specific position
        eprint_line(&lines, pos, &err.to_string())
    }
}


pub struct ScriptEngine{
    pub engine: Engine,
}
impl ScriptEngine{
    pub fn new() -> Self {
        let mut engine=Engine::new();
        engine.register_fn("add", add);

        ScriptEngine { engine: engine }
    }

    pub fn run_script(&mut self, filename: &str) -> Result<(), GenericError>{
        let mut contents = String::new();

        // #[cfg(not(feature = "no_optimize"))]
        // self.engine.set_optimization_level(OptimizationLevel::Full);

        let mut f =  File::open(&filename) ?;
        f.read_to_string(&mut contents)?;

        if let Err(err) = self.engine.consume(&contents) {
            eprintln!("{:=<1$}", "", filename.len());
            eprintln!("{}", filename);
            eprintln!("{:=<1$}", "", filename.len());
            eprintln!("");

            let info=err.to_string();
            eprint_error(&contents, *err);

            return Err(GenericError::ScriptError {
                file_name:filename.to_string(),
                info: info});
        }

        Ok(())
    }
}

fn add(x: INT, y: INT) -> INT {
    x + y
}

#[test]
fn run_script_works() -> anyhow::Result<()> {
    let mut runner=ScriptEngine::new();
    // runner.run_script("./src/scripts/simple.rhai")?;
    runner.run_script("./src/scripts/while.rhai")?;

    Ok(())
}

#[test]
fn script_time_check_works() -> anyhow::Result<()> {
    let mut runner=ScriptEngine::new();
    runner.run_script("./src/scripts/simple.rhai")?;

    Ok(())
}

#[test]
// fn eval_works() -> Result<(), Box<EvalAltResult>> // if not turn on 'sync' feature
fn eval_works() -> anyhow::Result<()>  {
    let s="add(40, 2)";
    let mut runner=ScriptEngine::new();

    let mut scope = Scope::new();
    let r=runner.engine.eval_with_scope::<INT>(&mut scope, s)?;
    println!("result: {}", r);
    Ok(())
}

