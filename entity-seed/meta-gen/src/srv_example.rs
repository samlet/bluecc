use chrono::{DateTime, Utc};
use crate::params::Object;
use crate::{SrvDeles, SrvResp, GenericError, DynamicValue};

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

/// The parameters for `Example::update`.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExample<'a> {
    pub example_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_type_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_name: Option<&'a str>,
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

impl<'a> UpdateExample<'a> {
    pub fn new(example_id: &'a str) -> Self {
        UpdateExample{
            example_id,
            example_type_id: Default::default(),
            status_id: Default::default(),
            example_name: Default::default(),
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
pub struct UpdateExampleResp {
    pub old_status_id: String,
}

pub type Response<T> = Result<SrvResp<T>, GenericError>;
impl Example{
    /// Creates a new example object.
    pub async fn create(dele: &SrvDeles, params: CreateExample<'_>) -> Response<ExampleId> {
        dele.srv("createExample", &params).await
    }

    pub async fn update(dele: &SrvDeles, params: UpdateExample<'_>) -> Response<UpdateExampleResp> {
        dele.srv("updateExample", &params).await
    }

    pub async fn delete(dele: &SrvDeles, params: &ExampleId) -> Response<DynamicValue> {
        dele.srv("deleteExample", &params).await
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

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

    #[tokio::test]
    async fn update_ex_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        // create
        let mut params = CreateExample::new(
            "CONTRIVED",
            "EXST_IN_DESIGN",
            "Example 2");
        params.description=Some("this is example 2");

        // let ret: SrvResp<ExampleId> = dele.srv("createExample", &params).await?;
        let ret = Example::create(&dele, params).await?;
        let ex_id=ret.data.unwrap().example_id;
        println!("created {}", ex_id);

        // update
        let mut params = UpdateExample::new(ex_id.as_str());
        params.example_name=Some("changed name");
        // let ret: SrvResp<UpdateExampleResp> = dele.srv("updateExample", &params).await?;
        let ret = Example::update(&dele, params).await?;
        if ret.is_ok() {
            let sts_id = ret.data.unwrap().old_status_id;
            println!("the old status id {}", sts_id);
        }else{
            println!("{}", ret.pretty_str()?);
        }

        // delete: 因为secas中定义了当createExample成功返回后会自动调用createExampleStatus,
        // 所以无法在删除对应ExampleStatus记录前删除Example实例;
        let params=ExampleId{ example_id: ex_id };
        // let ret: SrvResp<DynamicValue> = dele.srv("deleteExample", &params).await?;
        let ret = Example::delete(&dele, &params).await?;
        if ret.is_ok() {
            println!("removed {}", params.example_id);
        }else{
            println!("fail to remove: {}", ret.pretty_str()?);
        }

        Ok(())
    }

}

