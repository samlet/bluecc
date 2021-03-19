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
    let result = delegator.find_all("StatusItem").await?;
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
