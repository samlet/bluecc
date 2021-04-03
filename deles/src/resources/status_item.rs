use crate::GenericError;
use crate::delegators::{Delegator, pretty, render};
use chrono::{DateTime, Utc};
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatusItem{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

impl StatusItem{
    pub async fn list_items(delegator:&Delegator, type_id:&str) -> crate::Result<Vec<StatusItem>>{
        let conditions = "status_type_id".equals(type_id);
        let rs: Vec<StatusItem> = delegator.list_for("StatusItem", conditions.into()).await?;
        Ok(rs)
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    use inflector::Inflector;
    use crate::delegators::get_values_from_node;
    use std::collections::HashMap;
    use crate::delegators::values::get_values_from_map;

    // $ bluecc seed -s StatusItem
    /*
    StatusItem (1683..1805)
        description = Closed
        sequence_id = 50
        status_code = CLOSED
        status_id = POSTX_CLOSED
        status_type_id = POSTX_STATUS
     */

    #[tokio::test]
    async fn list_works() -> Result<(), GenericError> {
        let delegator = Delegator::new().await?;
        let rs=StatusItem::list_items(&delegator, "POSTX_STATUS").await?;
        println!("total {}", rs.len());
        render(&rs)?;
        Ok(())
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct StatusItemSeed{
        // keys
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_id: Option<String>,
        // fields
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_type_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sequence_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[tokio::test]
    async fn from_xml_works() -> crate::Result<()> {
        let raw=r#"<StatusItem description="In Design" sequenceId="01" statusCode="IN_DESIGN" statusId="EXST_IN_DESIGN" statusTypeId="EXAMPLE_STATUS"/>"#;
        let val_obj:StatusItemSeed=seed::load_xml(raw.as_bytes());
        println!("{}", pretty(&val_obj));
        let xml=serde_xml_rs::to_string(&val_obj)?;
        println!("{}", xml);
        Ok(())
    }

    #[tokio::test]
    async fn store_from_xml_works() -> crate::Result<()> {
        let raw=r#"<StatusItem description="In Design" sequenceId="01"
            statusCode="IN_DESIGN" statusId="bluecc_IN_DESIGN"
            statusTypeId="EXAMPLE_STATUS"/>"#;
        let doc = roxmltree::Document::parse(raw).unwrap();
        let node=doc.root_element();
        let (cols, store_values)=get_values_from_node(&node)?;

        println!("{:?}", cols);
        println!("{:?}", store_values);

        let table="status_item";
        let delegator = Delegator::new().await?;

        // let insert: Insert<'_> = Insert::multi_into(table, cols)
        //     .values(store_values).into();
        // // delegator.conn.insert(insert.into()).await?;
        // let changes = delegator.conn.execute(
        //     insert.on_conflict(OnConflict::DoNothing).into()).await?;
        // println!("changes: {}", changes);

        let insert: Insert<'_> = Insert::multi_into(table, cols)
            .values(store_values).into();
        let res = delegator.conn.insert(
            insert.on_conflict(OnConflict::DoNothing)
                .returning(vec![
                    "statusCode".to_snake_case(),
                    "createdTxStamp".to_snake_case()
                ])).await?;
        println!("res: {:?}", res);

        Ok(())
    }

    #[tokio::test]
    async fn store_from_obj_works() -> crate::Result<()> {
        let json_vals= serde_json::from_value(json!({
                      "statusId": "new_EXST_IN_DESIGN",
                      "statusTypeId": "EXAMPLE_STATUS",
                      "statusCode": "IN_DESIGN",
                      "sequenceId": "01",
                      "description": "In Design"
                }))?;
        let (cols,vals)=get_values_from_map(&json_vals)?;
        println!("{:?} -> \n{:?}", cols, vals);

        let delegator = Delegator::new().await?;
        let changes = delegator.store("StatusItem", &json_vals).await?;
        println!("changes: {}", changes);

        Ok(())
    }
}


