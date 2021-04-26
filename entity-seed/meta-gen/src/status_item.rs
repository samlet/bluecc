use crate::DynamicValue;
use chrono::{DateTime, Utc};
use crate::params::Object;

/// $ meta-cli entity StatusItem
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusItem{
    #[serde(flatten)]
    pub id: StatusItemId,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusItemId {
    pub status_id: Option<String>,
}

impl Object for StatusItem{
    type Id = StatusItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "status_item"
    }
}

/// $ meta-cli entity StatusValidChange
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusValidChange{
    #[serde(flatten)]
    pub id: StatusValidChangeId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_name: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusValidChangeId {
    pub status_id: Option<String>,
    pub status_id_to: Option<String>,
}

impl Object for StatusValidChange{
    type Id = StatusValidChangeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "status_valid_change"
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenericValues {
    result: Vec<DynamicValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusValidChanges {
    result: Vec<StatusValidChange>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::{SrvDeles, SrvResp};
    use serde_json::json;

    #[tokio::test]
    async fn perform_find_status_item_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusItem",
            "maxRows": 10000
        }))?;

        let ret: SrvResp<GenericValues> = dele.srv("findCc", &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn find_status_valid_change_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusValidChange",
            "maxRows": 10000
        }))?;

        let ret: SrvResp<StatusValidChanges> = dele.srv("findCc", &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        Ok(())
    }
}

