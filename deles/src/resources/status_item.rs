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
}
