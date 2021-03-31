use serde_json::Value;
use serde_json::json;
use bigdecimal::BigDecimal;
use std::convert::TryFrom;
use crate::GenericError;
use std::borrow::Cow;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};

#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue<'a> {
    Null,
    /// 64-bit signed integer.
    Integer(i64),
    /// 32-bit floating point.
    Float(f32),
    /// 64-bit floating point.
    Double(f64),
    /// String value.
    Text(String),
    /// Bytes value.
    Bytes(Cow<'a, [u8]>),
    /// Boolean value.
    Boolean(bool),
    /// A single character
    Char(char),
    /// An array value
    Array(Vec<AttrValue<'a>>),
    /// A numeric value.
    Numeric(BigDecimal),
    /// A JSON value.
    Json(serde_json::Value),
    /// A XML value.
    Xml(String),
    /// A datetime value.
    DateTime(Option<DateTime<Utc>>),
    /// A date value.
    Date(Option<NaiveDate>),
    /// A time value.
    Time(Option<NaiveTime>),
}

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

