use chrono::{DateTime, Utc};
use crate::params::Object;
use crate::{SrvDeles, SrvResp, GenericError, DynamicValue};
use std::collections::HashMap;

/// $ meta-cli resource findProductByIdCc plugins/adapters

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FindProductByIdCcReq<'a> {
    pub id_to_find: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub good_identification_type_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_product_first: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_all_id: Option<&'a str>,
}

impl<'a> FindProductByIdCcReq<'a> {
    pub fn new(id_to_find: &'a str) -> Self {
        FindProductByIdCcReq {
            id_to_find,
            good_identification_type_id: Default::default(),
            search_product_first: Default::default(),
            search_all_id: Default::default(),
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct FindProductByIdCcResp {
    pub product: Option<HashMap<String, serde_json::Value>>,
    pub products_list: Option<Vec<serde_json::Value>>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[tokio::test]
    async fn find_prod_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let mut params = FindProductByIdCcReq::new("GZ-1000",);
        let ret: SrvResp<FindProductByIdCcResp> = dele.srv("findProductByIdCc", &params).await?;
        println!("{}", ret.pretty_str()?);

        Ok(())
    }
}

