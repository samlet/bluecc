use serde_json::Value;
use serde_json::json;
use bigdecimal::BigDecimal;
use std::convert::TryFrom;
use crate::GenericError;
use std::borrow::Cow;
use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime};
use std::str::FromStr;
use inflector::Inflector;
use std::collections::HashMap;

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

pub fn convert_value(val: &serde_json::Value) -> crate::Result<Option<quaint::Value>> {
    let res=match val{
        Value::Null => {None}
        Value::Bool(v) => {Some(quaint::Value::boolean(v.to_owned()))}
        Value::Number(v) => {
            if v.is_f64() {Some(quaint::Value::double(v.as_f64().unwrap()))}
            else if v.is_i64() {Some(quaint::Value::integer(v.as_i64().unwrap()))}
            else {Some(quaint::Value::numeric(BigDecimal::try_from(v.as_f64().unwrap())?))}
        }
        Value::String(v) => {Some(quaint::Value::text(v))}
        Value::Array(v) => {Some(quaint::Value::array(v.to_owned()))}
        Value::Object(_) => {Some(quaint::Value::json(val.to_owned()))}
    } ;
    Ok(res)
}

fn convert_field_value<'a>(fld_type:&str, fld_val:String) -> crate::Result<quaint::Value<'a>>{
    use chrono::format::strftime::StrftimeItems;
    use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

    const STD_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

    let parse_dt=NaiveDateTime::parse_from_str;

    let store_val=match fld_type {
            "date-time" => {quaint::Value::datetime(
                DateTime::<Utc>::from_utc(
                    parse_dt(fld_val.as_str(), STD_FORMAT)?, Utc))}
            "date" => {quaint::Value::date(NaiveDate::parse_from_str(fld_val.as_str(), "%Y-%m-%d")?)}
            "time" => {quaint::Value::time(NaiveTime::parse_from_str(fld_val.as_str(), "%H:%M:%S")?)}
            "blob"|"byte-array" => {
                quaint::Value::Bytes(Some(Cow::Owned(fld_val.as_bytes().into())))
            }
            "currency-amount" | "currency-precise" | "fixed-point"=>
                {quaint::Value::numeric(BigDecimal::from_str(fld_val.as_str())?)}
            "floating-point" => {quaint::Value::double(fld_val.parse()?)}
            "integer" | "numeric" => {quaint::Value::integer(fld_val.parse::<i64>()?)}
            "indicator" => {quaint::Value::character(fld_val.chars().next().unwrap())}
            _ => {quaint::Value::text(fld_val.to_owned())}
        };
    Ok(store_val)
}

pub fn get_values_from_node<'a>(node: &'a roxmltree::Node) -> crate::Result<(Vec<String>, Vec<quaint::Value<'a>>)> {
    let ent_name=node.tag_name().name();
    let meta=seed::get_entity_model(ent_name)?;
    let mut cols=Vec::new();
    let mut store_values=Vec::new();
    for f in node.attributes().iter(){
        let fld_val=f.value();
        // let fld_val= f.value().to_owned();
        let fld_name=f.name();
        cols.push(fld_name.to_snake_case());
        let fld=meta.get_field(fld_name).expect("field-model");
        let store_val=convert_field_value(fld.field_type.as_str(), fld_val.to_owned())?;
        store_values.push(store_val);
    }

    Ok((cols, store_values))
}

pub fn get_values_from_map(map_vals: &serde_json::Map<String, serde_json::Value>)
                           -> crate::Result<(Vec<String>, Vec<quaint::Value>)> {
    // let meta=seed::get_entity_model(ent_name)?;
    let mut cols=Vec::new();
    let mut store_values=Vec::new();
    for (fld_name, f) in map_vals{
        cols.push(fld_name.to_snake_case());
        // let fld=meta.get_field(fld_name).expect("field-model");
        // let store_val=convert_field_value(fld.field_type.as_str(), fld_val.to_owned())?;
        let store_val=convert_value(f)?.unwrap();
        store_values.push(store_val);
    }

    Ok((cols, store_values))
}

pub fn get_values_from_string_map(ent_name: &str, map_vals: HashMap<String, String>)
                           -> crate::Result<(Vec<String>, Vec<quaint::Value>)> {
    let meta=seed::get_entity_model(ent_name)?;
    let mut cols=Vec::new();
    let mut store_values=Vec::new();
    for (fld_name, fld_val) in map_vals{
        cols.push(fld_name.to_snake_case());
        let fld=meta.get_field(fld_name.as_str()).expect("field-model");
        let store_val=convert_field_value(fld.field_type.as_str(), fld_val.to_owned())?;
        store_values.push(store_val);
    }

    Ok((cols, store_values))
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn json_value_works() -> crate::Result<()> {
        // let val=serde_json::Value::from(64);
        let val = json!(12.5);
        let res = convert_value(&val)?;
        println!("get {:?}", res);
        Ok(())
    }

    #[test]
    fn from_map_works() -> crate::Result<()> {
        let json_vals= serde_json::from_value(json!({
                    "productId":"Product",
                    "taxable":"N",
                }))?;
        let (cols,vals)=get_values_from_map(&json_vals)?;
        println!("{:?} -> {:?}", cols, vals);
        Ok(())
    }

    #[test]
    fn char_works() -> anyhow::Result<()> {
        let fld_val="Y";
        let c=fld_val.chars().next().unwrap();
        println!("{}", c);
        Ok(())
    }
}


