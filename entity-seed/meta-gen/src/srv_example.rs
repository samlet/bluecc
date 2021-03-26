use chrono::{DateTime, Utc};
use crate::params::Object;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Example {
    #[serde(flatten)]
    pub id: ExampleId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ExampleId{
    pub example_id: String,
}

impl Object for Example {
    type Id = ExampleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "example"
    }
}

/// The parameters for `Example::create`.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateExample<'a> {
    pub example_type_id: &'a str,
    pub status_id: &'a str,
    pub example_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub another_text: Option<&'a str>,
}

impl<'a> CreateExample<'a> {
    pub fn new(example_type_id: &'a str, status_id: &'a str, example_name: &'a str) -> Self {
        CreateExample{
            example_type_id,
            status_id,
            example_name,
            description: Default::default(),
            long_description: Default::default(),
            comments: Default::default(),
            example_size: Default::default(),
            example_date: Default::default(),
            another_date: Default::default(),
            another_text: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct CreateExampleResp {
    pub example_id: String,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::{SrvDeles, SrvResp, GenericError, DynamicValue};

    #[tokio::test]
    async fn create_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let mut params = CreateExample::new(
            "CONTRIVED",
            "EXST_IN_DESIGN",
            "Example 1");

        let ret: SrvResp<ExampleId> = dele.srv("createExample", &params).await?;
        // with DynamicValue
        // let ret: SrvResp<DynamicValue> = dele.srv("createExample", &params).await?;
        let data_json = serde_json::to_string_pretty(&ret)?;
        println!("{}", data_json);

        Ok(())
    }

}

