use crate::GenericError;
use crate::delegators::{Delegator, pretty, render};
use chrono::{DateTime, Utc};
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};

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
    pub async fn list_items(delegator:&Delegator, type_id:&str) -> Result<Vec<StatusItem>, GenericError>{
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
        Ok(())
    }

    #[tokio::test]
    async fn store_from_xml_works() -> crate::Result<()> {
        use chrono::format::strftime::StrftimeItems;
        use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

        let parse_dt=NaiveDateTime::parse_from_str;

        let raw=r#"<StatusItem description="In Design" sequenceId="01" statusCode="IN_DESIGN" statusId="EXST_IN_DESIGN" statusTypeId="EXAMPLE_STATUS"/>"#;
        let doc = roxmltree::Document::parse(raw).unwrap();
        let node=doc.root_element();
        let meta=seed::get_entity_model("StatusItem")?;
        let mut cols=Vec::new();
        let mut store_values=Vec::new();
        for f in node.attributes().iter(){
            let fld_val=f.value();
            let fld_name=f.name();
            cols.push(fld_name.to_snake_case());
            let fld=meta.get_field(fld_name).expect("field-model");
            let store_val=match fld.field_type.as_str() {
                "date-time" => {quaint::Value::datetime(
                    DateTime::<Utc>::from_utc(
                        parse_dt(fld_val, "%Y-%m-%d %H:%M:%S")?, Utc))}
                "date" => {quaint::Value::date(NaiveDate::parse_from_str(fld_val, "%Y-%m-%d")?)}
                "time" => {quaint::Value::time(NaiveTime::parse_from_str(fld_val, "%H:%M:%S")?)}
                "blob"|"byte-array" => {quaint::Value::bytes(fld_val.as_bytes())}
                "currency-amount" | "currency-precise" | "fixed-point"=>
                    {quaint::Value::numeric(BigDecimal::from_str(fld_val)?)}
                "floating-point" => {quaint::Value::double(fld_val.parse()?)}
                "integer" | "numeric" => {quaint::Value::integer(fld_val.parse::<i64>()?)}
                "indicator" => {quaint::Value::character(fld_val.chars().next().unwrap())}
                _ => {quaint::Value::text(fld_val)}
            };
            store_values.push(store_val);
        }

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
}


