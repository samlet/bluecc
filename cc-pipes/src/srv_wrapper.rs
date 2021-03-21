use rhai::{Engine, EvalAltResult, RegisterFn, INT, RegisterResultFn};
use rhai::{Dynamic, Map};
use rhai::serde::{to_dynamic, from_dynamic};
use std::collections::HashMap;
use serde_json::Value;
use futures::executor::block_on;
use meta_gen::{SrvDeles, SrvResp, DynamicValue, GenericError};

#[derive(Clone)]
struct SrvWrapper {
    deles: SrvDeles,
    field: i64
}

impl SrvWrapper {
    fn new() -> Self {
        Self { deles: SrvDeles::new(), field: 1 }
    }

    fn update(&mut self, x: i64) {      // methods take &mut as first parameter
        self.field += x;
    }

    fn foo(&mut self, params: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut input: DynamicValue = from_dynamic(&params)?;
        let val=&input.values.get("a").unwrap().as_i64().unwrap();
        println!("input is {:?}", input);
        input.values.insert("b".to_string(), Value::from("resp"));
        let result: Dynamic = to_dynamic(input)?;

        self.field+=val;
        Ok(result)
    }
}

trait CommonSrvs{
    fn test_scv(&mut self, params: Dynamic) -> Result<Dynamic, Box<EvalAltResult>>;
}

impl CommonSrvs for SrvWrapper{
    fn test_scv(&mut self, params: Dynamic) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut input: DynamicValue = from_dynamic(&params)?;
        let val=&input.values.get("a").unwrap().as_i64().unwrap();
        println!("input is {:?}", input);
        input.values.insert("b".to_string(), Value::from("resp"));
        let result: Dynamic = to_dynamic(input)?;

        self.field+=val;
        Ok(result)
    }
}

async fn srv_invoke_with_dynamic_works() -> Result<String, GenericError> {
    let mut dele=SrvDeles::new();
    dele.use_default_token().await?;
    println!("tok {}", dele.access_token);

    let mut values=HashMap::new();
    values.insert("defaultValue".to_string(), Value::from(8.8));
    values.insert("message".to_string(), Value::from("hi"));
    let ret: SrvResp<DynamicValue>=dele.srv("testScv", &DynamicValue{ values: values,}).await?;
    let data_json=serde_json::to_string_pretty(&ret)?;

    Ok(data_json)
}

#[test]
fn block_srv_test() {
    use tokio::runtime::Runtime;
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    rt.block_on(async {
        let resp = srv_invoke_with_dynamic_works().await;
        println!("resp -> {}", resp.expect("connect fail"));
    });
}


#[test]
fn wrapper_works() -> anyhow::Result<()> {
    let mut engine = Engine::new();

    // Most Engine API's can be chained up.
    engine.register_type::<SrvWrapper>()    // register custom type
        .register_fn("new_ts", SrvWrapper::new)
        .register_fn("update", SrvWrapper::update)
        .register_result_fn("foo", SrvWrapper::foo)
        .register_result_fn("test_scv", SrvWrapper::test_scv);

    // Cast result back to custom type.
    let result = engine.eval::<SrvWrapper>(
        r#"
        let x = new_ts();               // calls 'SrvWrapper::new'
        x.update(41);                   // calls 'SrvWrapper::update'
        let resp_comm=x.foo(#{ a: 42, b: [ "hello", "world" ], c: true, });
        let resp_srv=x.test_scv(#{ a: 42, b: [ "hello", "world" ], c: true, });
        x                               // 'x' holds a 'SrvWrapper'
    "#)?;

    println!("result: {}", result.field);   // prints 42

    Ok(())
}

