use serde_json::Value;
use toml;
use std::any::Any;

#[derive(Serialize, Deserialize)]
struct Params {
    a: i32,
    b: i32,
}
#[derive(Serialize, Deserialize)]
enum Message {
    Request { id: String, method: String, params: Params },
    Response { id: String, result: Value },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum Block {
    Para(Vec<Params>),
    Str(String),
}

#[test]
fn enum_works() -> anyhow::Result<()> {
    #[derive(Debug, Serialize, Deserialize)]
    enum E {
        W { a: i32, b: i32 },
        X(i32, i32),
        Y(i32),
        Z,
    }
    let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
    let x = E::X(0, 0);          // Represented as `{"X":[0,0]}`
    let y = E::Y(0);             // Represented as `{"Y":0}`
    let z = E::Z;                // Represented as `"Z"`
    println!("{:?} {:?} {:?} {:?}", w,x,y,z);
    Ok(())
}

// type usize=u32;
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "args")]
enum Actions {
    Wait(usize),
    Move { x: usize, y: usize },
}

// https://stackoverflow.com/questions/48641541/deserializing-toml-into-vector-of-enum-with-values
#[test]
fn toml_works() -> anyhow::Result<()> {
    let a_wait = Actions::Wait(5);
    println!("{}", toml::to_string(&a_wait).unwrap());

    let a_move = Actions::Move { x: 1, y: 1 };
    println!("{}", toml::to_string(&a_move).unwrap());

    let config: Actions = toml::from_str(r#"
        type = "Move"

        [args]
        x = 1
        y = 1
    "#).unwrap();
    // println!("{}", config.type_id());
    match config {
        Actions::Move { x, y } => {
            assert_eq!(x, 1);
            assert_eq!(y, 1);
        }
        _ => ()
    };

    Ok(())
}

#[test]
fn bigdecimal_works() -> anyhow::Result<()> {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    let input = "0.8";
    let dec = BigDecimal::from_str(&input).unwrap();
    let float = f32::from_str(&input).unwrap();

    println!("Input ({}) with 10 decimals: {} vs {})", input, dec, float);

    Ok(())
}