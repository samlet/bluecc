use serde_json::Value;
use serde_json::json;
use bigdecimal::BigDecimal;
use std::convert::TryFrom;
use crate::GenericError;

pub fn convert_value<'a>(val:serde_json::Value) -> crate::Result<Option<quaint::Value<'a>>>{
    let res=match val{
        Value::Null => {None}
        Value::Bool(v) => {Some(quaint::Value::boolean(v))}
        Value::Number(v) => {
            if v.is_f64() {Some(quaint::Value::double(v.as_f64().unwrap()))}
            else if v.is_i64() {Some(quaint::Value::integer(v.as_i64().unwrap()))}
            else {Some(quaint::Value::numeric(BigDecimal::try_from(v.as_f64().unwrap())?))}
        }
        Value::String(v) => {Some(quaint::Value::text(v))}
        Value::Array(v) => {Some(quaint::Value::array(v))}
        Value::Object(_) => {Some(quaint::Value::json(val))}
    } ;
    Ok(res)
}

#[test]
fn json_value_works() -> crate::Result<()> {
    // let val=serde_json::Value::from(64);
    let val = json!(12.5);
    let res=convert_value(val)?;
    println!("get {:?}", res);
    Ok(())
}

