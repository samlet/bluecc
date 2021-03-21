use rhai::{Engine, EvalAltResult, RegisterFn, INT, RegisterResultFn};
use rhai::{Dynamic, Map};
use rhai::serde::{to_dynamic, from_dynamic};
use std::collections::HashMap;
use serde_json::Value;
use futures::executor::block_on;

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

/// https://rhai.rs/book/rust/serde.html
#[test]
fn dynamic_works() -> anyhow::Result<()> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct MyStruct {
        a: i64,
        b: Vec<String>,
        c: bool,
        d: Point
    }

    let x = MyStruct {
        a: 42,
        b: vec![ "hello".into(), "world".into() ],
        c: true,
        d: Point { x: 123.456, y: 999.0 }
    };

    // Convert the 'MyStruct' into a 'Dynamic'
    let map: Dynamic = to_dynamic(x)?;
    assert!(map.is::<Map>());
    println!("{}", map.type_name());

    let engine = Engine::new();
    let result: Dynamic = engine.eval(r#"
                #{
                    a: 42,
                    b: [ "hello", "world" ],
                    c: true,
                    d: #{ x: 123.456, y: 999.0 }
                }
            "#)?;

    // Convert the 'Dynamic' object map into 'MyStruct'
    let x: MyStruct = from_dynamic(&result)?;
    println!("{:?}", x);
    Ok(())
}

#[test]
fn test_map_return() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    let x = engine.eval::<Map>(r#"#{a: 1, b: true, "c$": "hello"}"#)?;

    assert_eq!(x["a"].clone().cast::<INT>(), 1);
    assert_eq!(x["b"].clone().cast::<bool>(), true);
    assert_eq!(x["c$"].clone().cast::<String>(), "hello");

    Ok(())
}

#[test]
fn test_map_oop() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(
        engine.eval::<INT>(
            r#"
                let obj = #{ data: 40, action: Fn("abc") };

                fn abc(x) { this.data += x; }

                obj.action(2);
                obj.data
            "#,
        )?,
        42
    );

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct SrvParams {
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[test]
fn dynamic_flatten_works() -> anyhow::Result<()> {
    let engine = Engine::new();
    let result: Dynamic = engine.eval(r#"
                #{
                    a: 42,
                    b: [ "hello", "world" ],
                    c: true,
                }
            "#)?;

    // Convert the 'Dynamic' object map into 'MyStruct'
    let x: SrvParams = from_dynamic(&result)?;
    println!("{:?}", x);
    Ok(())
}

fn foo(params: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>  {
    let mut input: SrvParams = from_dynamic(&params)?;
    println!("input is {:?}", input);
    input.extra.insert("b".to_string(), Value::from("resp"));
    let result: Dynamic = to_dynamic(input)?;
    Ok(result)
}

#[test]
fn fn_works() -> anyhow::Result<()> {
    let mut engine = Engine::new();
    engine.register_result_fn("foo", foo);

    let result: Dynamic = engine.eval(r#"
                let result=foo(#{ a: 42, b: [ "hello", "world" ], c: true, });
                result
            "#)?;
    let output: SrvParams = from_dynamic(&result)?;
    println!("output is {:?}", output);
    Ok(())
}

// Function that may fail - the result type must be 'Dynamic'
fn safe_divide(x: i64, y: i64) -> Result<Dynamic, Box<EvalAltResult>> {
    if y == 0 {
        // Return an error if y is zero
        Err("Division by zero!".into())         // shortcut to create Box<EvalAltResult::ErrorRuntime>
    } else {
        Ok((x / y).into())                      // convert result into 'Dynamic'
    }
}

#[test]
fn register_works() -> anyhow::Result<()> {
    let mut engine = Engine::new();
    // Fallible functions that return Result values must use register_result_fn()
    engine.register_result_fn("divide", safe_divide);

    if let Err(error) = engine.eval::<i64>("let result=divide(40, 0); result") {
        println!("Error: {:?}", *error);         // prints ErrorRuntime("Division by zero detected!", (1, 1)")
    }

    let result=engine.eval::<i64>("let result=divide(40, 2); result")?;
    println!("result is {}", result);

    Ok(())
}

#[derive(Clone)]
struct TestCls {
    field: i64
}

impl TestCls {
    fn new() -> Self {
        Self { field: 1 }
    }

    fn update(&mut self, x: i64) {      // methods take &mut as first parameter
        self.field += x;
    }

    fn foo(&mut self, params: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut input: SrvParams = from_dynamic(&params)?;
        let val=&input.extra.get("a").unwrap().as_i64().unwrap();
        println!("input is {:?}", input);
        input.extra.insert("b".to_string(), Value::from("resp"));
        let result: Dynamic = to_dynamic(input)?;

        self.field+=val;
        Ok(result)
    }
}

#[test]
fn cls_works() -> anyhow::Result<()> {
    let mut engine = Engine::new();

    // Most Engine API's can be chained up.
    engine.register_type::<TestCls>()    // register custom type
        .register_fn("new_ts", TestCls::new)
        .register_fn("update", TestCls::update)
        .register_result_fn("foo", TestCls::foo);

    // Cast result back to custom type.
    let result = engine.eval::<TestCls>(
        r#"
        let x = new_ts();               // calls 'TestCls::new'
        x.update(41);                   // calls 'TestCls::update'
        let result=x.foo(#{ a: 42, b: [ "hello", "world" ], c: true, });
        x                               // 'x' holds a 'TestCls'
    "#)?;

    println!("result: {}", result.field);   // prints 42

    Ok(())
}

async fn hello_world() -> anyhow::Result<String> {
    println!("hello, world!");
    Ok("it's ok".to_string())
}

#[test]
fn block_test() {
    let future = hello_world(); // Nothing is printed
    let resp=block_on(future); // `future` is run and "hello, world!" is printed
    println!("resp -> {}", resp.unwrap());
}


