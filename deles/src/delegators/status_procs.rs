use crate::delegators::Delegator;
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use seed::GenericError;
use inflector::Inflector;
use std::collections::HashMap;
use futures::StreamExt;
use log::kv::{Source, ToKey};
use serde_json::Value;

#[tokio::test]
async fn status_item_works() -> anyhow::Result<()> {
    let internal_fields=vec!["created_stamp", "created_tx_stamp",
                                 "last_updated_stamp", "last_updated_tx_stamp"];

    let delegator = Delegator::new().await?;
    let result = delegator.find_all("StatusItem", false, false).await?;
    let mut rs=Vec::new();
    let cols=result.rs.columns().to_owned();
    for row in result.rs.into_iter() {
        // println!("{:?}", row);
        let mut row_map:HashMap<String,serde_json::Value> = HashMap::new();
        for (idx, p_value) in row.into_iter().enumerate() {
            let column_name = &cols[idx];
            let val = serde_json::Value::from(p_value);
            if let serde_json::Value::Null = val {
                continue;
            }
            if !internal_fields.contains(&column_name.as_str()) {
                row_map.insert(column_name.to_camel_case(), val);
            }
        }

        rs.push(row_map);
    }

    println!("total {}", rs.len());
    let ex_sts=rs.iter().filter(|&m|
        m.get("statusTypeId").unwrap()==&Value::from("EXAMPLE_STATUS"))
        .collect::<Vec<&HashMap<String,Value>>>();
    for item in &ex_sts{
        println!("item -> {} ({})", item.get("description").unwrap(),
                 item.get("statusId").unwrap());
        for (k,v) in item.iter() {
            println!("\t{} {}", k, v);
        }
    }
    println!("total ex_sts {}", ex_sts.len());
    Ok(())
}

// generate by: $ cargo run --bin seed gen StatusItem dto_orig
// 使用camelCase是因为GenericValues在转换时将列名由snake_case转换成了camelCase,
// 为了与模型字段名保持统一
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatusItem{
    // keys
    pub status_id: Option<String>,
    // fields
    pub status_type_id: Option<String>,
    pub status_code: Option<String>,
    pub sequence_id: Option<String>,
    pub description: Option<String>
}

async fn print_sts(status_type: &str, items: &Vec<StatusItem>) -> anyhow::Result<()> {
    let ex_sts:Vec<&StatusItem>=items.iter()
        .filter(|&n|n.status_type_id==Some(status_type.to_string()))
        .collect();
    for ex in &ex_sts{
        println!("{}", ex.description.as_ref().unwrap())
    }
    Ok(())
}

#[tokio::test]
async fn serialize_json_works() -> anyhow::Result<()> {
    let delegator=Delegator::new().await?;
    let rs=delegator.find_all("StatusItem", true, true).await?;
    let jval=serde_json::Value::from(rs);
    let rows=jval.as_array();
    println!("total {}", rows.unwrap().len());

    let mut items=Vec::new();
    for row in rows.unwrap() {
        // println!("{:?}", row);
        let v=serde_json::from_value::<StatusItem>(row.to_owned())?;
        let rec_json= serde_json::to_string_pretty(&v)?;
        println!("{}", rec_json);

        items.push(v);
    }

    let status_type="EXAMPLE_STATUS";
    print_sts(status_type, &items).await?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatusItemRaw{
    // keys
    pub status_id: Option<String>,
    // fields
    pub status_type_id: Option<String>,
    pub status_code: Option<String>,
    pub sequence_id: Option<String>,
    pub description: Option<String>
}

async fn print_raw_sts(status_type: &str, items: &Vec<StatusItemRaw>) -> anyhow::Result<()> {
    let ex_sts:Vec<&StatusItemRaw>=items.iter()
        .filter(|&n|n.status_type_id==Some(status_type.to_string()))
        .collect();
    for ex in &ex_sts{
        println!("{}", ex.description.as_ref().unwrap())
    }
    Ok(())
}

#[tokio::test]
async fn list_ent_works() -> Result<(), GenericError> {
    let delegator=Delegator::new().await?;
    let rs:Vec<StatusItemRaw>=delegator.list("StatusItem").await?;
    println!("total {}", rs.len());
    let status_type="EXAMPLE_STATUS";
    print_raw_sts(status_type, &rs).await?;
    Ok(())
}

