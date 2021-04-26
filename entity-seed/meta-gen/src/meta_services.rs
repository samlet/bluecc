use crate::{SrvResp, DynamicValue, SrvDeles};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct EntityMeta {
    pub name: String,
    pub package_name: String,
    pub description: Option<String>,
    pub fields: Vec<FieldMeta>,
    pub relations: Vec<RelationMeta>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct FieldMeta{
    pub name: String,
    #[serde(rename = "type", default)]
    pub field_type: String,
    pub pk: bool,
    pub not_null: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct RelationMeta {
    pub name: String,
    #[serde(rename = "type", default)]
    pub rel_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct RetrieveEntityMeta{
    pub entity: EntityMeta,
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[tokio::test]
    async fn get_entity_meta_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        // retrieve view-entity meta
        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusValidChangeToDetail"
        }))?;

        let ret: SrvResp<RetrieveEntityMeta> = dele.srv("getEntityMeta", &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        Ok(())
    }
}
