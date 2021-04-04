use crate::resources::Resource;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmplPosition{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empl_position_id: Option<String>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_item_seq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empl_position_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_from_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_thru_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulltime_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_from_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_thru_date: Option<DateTime<Utc>>
}

impl Resource for EmplPosition{
    const KIND: &'static str = "Entity";
    const NAME: &'static str = "EmplPosition";
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::GenericError;
    use crate::delegators::Delegator;

    #[tokio::test]
    async fn list_ent_works() -> crate::Result<()> {
        let delegator=Delegator::new().await?;
        let rs:Vec<EmplPosition>=delegator.list(EmplPosition::NAME).await?;
        println!("total {}", rs.len());
        rs.iter().for_each(|r|
            println!("{}", serde_json::to_string_pretty(r).unwrap()));
        Ok(())
    }
}

