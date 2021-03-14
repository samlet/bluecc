use rhai::{Engine, EvalAltResult, RegisterFn, INT};

#[test]
fn script_works() -> Result<(), Box<EvalAltResult>>
{
    let engine = Engine::new();

    let result = engine.eval::<i64>("40 + 2")?;
    //                      ^^^^^^^ cast the result to an 'i64', this is required
    println!("Answer: {}", result);             // prints 42
    Ok(())
}

#[derive(Debug, Clone)]
struct TestStruct {
    x: INT,
}

impl TestStruct {
    pub fn update(&mut self) {
        self.x += 1000;
    }
    pub fn new() -> Self {
        Self { x: 1 }
    }
}

#[test]
fn register_fn_works() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine
        .register_type::<TestStruct>()
        .register_fn("new_ts", TestStruct::new)
        .register_fn("update", TestStruct::update);

    let result = engine.eval::<TestStruct>(
        r"
            let x = new_ts();
            x.update();
            x
        ",
    )?;

    println!("{:?}", result);

    let result = engine.eval::<TestStruct>(
        r"
            let x = [ new_ts() ];
            x[0].update();
            x[0]
        ",
    )?;

    println!("{:?}", result);

    Ok(())
}
