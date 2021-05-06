use chrono::{DateTime, Utc};
use crate::params::Object;
use crate::{SrvDeles, SrvResp, GenericError, DynamicValue};
use std::collections::HashMap;
use serde_json::json;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Deserializer};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PerformFindItemReq<'a> {
    pub entity_name: &'a str,
    pub input_fields: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_date: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by_date_value: Option<DateTime<Utc>>,
}

impl<'a> PerformFindItemReq<'a> {
    pub fn new(entity_name: &'a str, input_fields: HashMap<String, serde_json::Value>) -> Self {
        PerformFindItemReq {
            entity_name,
            input_fields,
            order_by: Default::default(),
            filter_by_date: Default::default(),
            filter_by_date_value: Default::default(),
        }
    }
}

const PERFORM_FIND_ITEM: &'static str = "performFindItem";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct PerformFindItemResp {
    pub item: Option<HashMap<String, serde_json::Value>>,
    pub query_string: Option<String>,
    pub query_string_map: Option<HashMap<String, serde_json::Value>>,
}

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

const FIND_PRODUCT_BY_ID_CC: &'static str = "findProductByIdCc";


pub async fn find_cc(dele: &SrvDeles, entity_name: &str, max_rows: usize)
    -> Result<SrvResp<DynamicValue>, GenericError> {
    let values: DynamicValue = serde_json::from_value(json!({
        "entityName": entity_name,
        "maxRows": max_rows,
    }))?;
    dele.srv("findCc", &values).await
}


#[cfg(test)]
mod lib_tests {
    use super::*;
    use inflector::Inflector;
    use crate::resources::product::Product;
    use crate::extract_val;
    use deles::delegators::pretty;
    use serde_json::Value;

    #[tokio::test]
    async fn find_prod_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let mut params = FindProductByIdCcReq::new("GZ-1000",);
        let ret: SrvResp<FindProductByIdCcResp> = dele.srv(FIND_PRODUCT_BY_ID_CC, &params).await?;
        println!("{}", ret.pretty_str()?);

        Ok(())
    }

    #[test]
    fn srv_name_works() -> anyhow::Result<()> {
        println!("{}", "findProductByIdCc".to_screaming_snake_case());
        Ok(())
    }

    #[tokio::test]
    async fn perform_find_list_works() -> Result<(), GenericError> {
        let mut dele=SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values:DynamicValue=serde_json::from_value(json!({
            "entityName":"Product",
            "viewIndex": 0,
            "viewSize": 10,
            "inputFields":{
                "productTypeId": "FINISHED_GOOD"
            }
        }))?;

        let ret: SrvResp<DynamicValue>=dele.srv("performFindList", &values).await?;
        println!("{}", ret.pretty_str()?);

        Ok(())
    }

    #[tokio::test]
    async fn perform_find_item_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"Product",
            "orderBy": "productTypeId",
            "inputFields":{
                "productId": "GZ-1000"
            }
        }))?;

        let ret: SrvResp<DynamicValue> = dele.srv("performFindItem", &values).await?;
        println!("{}", ret.pretty_str()?);

        assert!(ret.is_ok());

        // if let Some(item)=ret.data.unwrap().values.get("item"){
        //     let prod:Product=serde_json::from_value(item.clone())?;
        //     println!("product name {}", prod.product_name.unwrap());
        // }

        let prod:Product=extract_val(&ret, "item")?;
        println!("product name {}", prod.product_name.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn perform_find_item_ty_works() -> Result<(), GenericError> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let mut values: PerformFindItemReq = PerformFindItemReq::new(
            "Product", HashMap::new());
        values.input_fields.insert("productId".to_string(),
                                   serde_json::Value::from("GZ-1000"));

        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct ProductResp {
            pub item: Product,
        }
        let ret: SrvResp<ProductResp> = dele.srv(PERFORM_FIND_ITEM, &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        let prod_name=ret.data.expect("data").item.product_name.unwrap();
        println!("{}", prod_name);
        Ok(())
    }

    #[test]
    fn json_selector_works() -> anyhow::Result<()> {
        let json_obj = json!({
            "store": {
                "book": [
                    {
                        "category": "reference",
                        "author": "Nigel Rees",
                        "title": "Sayings of the Century",
                        "price": 8.95
                    },
                    {
                        "category": "fiction",
                        "author": "Evelyn Waugh",
                        "title": "Sword of Honour",
                        "price": 12.99
                    },
                    {
                        "category": "fiction",
                        "author": "Herman Melville",
                        "title": "Moby Dick",
                        "isbn": "0-553-21311-3",
                        "price": 8.99
                    },
                    {
                        "category": "fiction",
                        "author": "J. R. R. Tolkien",
                        "title": "The Lord of the Rings",
                        "isbn": "0-395-19395-8",
                        "price": 22.99
                    }
                ],
                "bicycle": {
                    "color": "red",
                    "price": 19.95
                }
            },
            "expensive": 10
        });

        let mut selector = jsonpath::selector(&json_obj);

        assert_eq!(selector("$.store.book[*].author").unwrap(),
                   vec![
                       "Nigel Rees", "Evelyn Waugh", "Herman Melville", "J. R. R. Tolkien"
                   ]);

        assert_eq!(selector("$..author").unwrap(),
                   vec![
                       "Nigel Rees", "Evelyn Waugh", "Herman Melville", "J. R. R. Tolkien"
                   ]);

        assert_eq!(selector("$.store.*").unwrap(),
                   vec![
                       &json!([
                    { "category": "reference", "author": "Nigel Rees", "title": "Sayings of the Century", "price": 8.95 },
                    { "category": "fiction", "author": "Evelyn Waugh", "title": "Sword of Honour", "price": 12.99 },
                    { "category": "fiction", "author": "Herman Melville", "title": "Moby Dick", "isbn": "0-553-21311-3", "price": 8.99 },
                    { "category": "fiction", "author": "J. R. R. Tolkien", "title": "The Lord of the Rings", "isbn": "0-395-19395-8", "price": 22.99 }
                ]),
                       &json!({ "color": "red", "price": 19.95 })
                   ]);

        assert_eq!(selector("$.store..price").unwrap(),
                   vec![
                       8.95, 12.99, 8.99, 22.99, 19.95
                   ]);

        assert_eq!(selector("$..book[2]").unwrap(),
                   vec![
                       &json!({
                    "category" : "fiction",
                    "author" : "Herman Melville",
                    "title" : "Moby Dick",
                    "isbn" : "0-553-21311-3",
                    "price" : 8.99
                })
                   ]);

        assert_eq!(selector("$..book[-2]").unwrap(),
                   vec![
                       &json!({
                    "category" : "fiction",
                    "author" : "Herman Melville",
                    "title" : "Moby Dick",
                    "isbn" : "0-553-21311-3",
                    "price" : 8.99
                })
                   ]);

        assert_eq!(selector("$..book[0,1]").unwrap(),
                   vec![
                       &json!({"category" : "reference","author" : "Nigel Rees","title" : "Sayings of the Century","price" : 8.95}),
                       &json!({"category" : "fiction","author" : "Evelyn Waugh","title" : "Sword of Honour","price" : 12.99})
                   ]);

        assert_eq!(selector("$..book[:2]").unwrap(),
                   vec![
                       &json!({"category" : "reference","author" : "Nigel Rees","title" : "Sayings of the Century","price" : 8.95}),
                       &json!({"category" : "fiction","author" : "Evelyn Waugh","title" : "Sword of Honour","price" : 12.99})
                   ]);

        assert_eq!(selector("$..book[:2]").unwrap(),
                   vec![
                       &json!({"category" : "reference","author" : "Nigel Rees","title" : "Sayings of the Century","price" : 8.95}),
                       &json!({"category" : "fiction","author" : "Evelyn Waugh","title" : "Sword of Honour","price" : 12.99})
                   ]);

        assert_eq!(selector("$..book[?(@.isbn)]").unwrap(),
                   vec![
                       &json!({"category" : "fiction","author" : "Herman Melville","title" : "Moby Dick","isbn" : "0-553-21311-3","price" : 8.99}),
                       &json!({"category" : "fiction","author" : "J. R. R. Tolkien","title" : "The Lord of the Rings","isbn" : "0-395-19395-8","price" : 22.99})
                   ]);

        assert_eq!(selector("$.store.book[?(@.price < 10)]").unwrap(),
                   vec![
                       &json!({"category" : "reference","author" : "Nigel Rees","title" : "Sayings of the Century","price" : 8.95}),
                       &json!({"category" : "fiction","author" : "Herman Melville","title" : "Moby Dick","isbn" : "0-553-21311-3","price" : 8.99})
                   ]);

        Ok(())
    }

    #[test]
    fn select_as_works() -> anyhow::Result<()> {
        let json_obj = json!({
            "school": {
                "friends": [
                    {"name": "친구1", "age": 20},
                    {"name": "친구2", "age": 20}
                ]
            },
            "friends": [
                {"name": "친구3", "age": 30},
                {"name": "친구4"}
        ]});

        #[derive(Deserialize, PartialEq, Debug)]
        struct Friend {
            name: String,
            age: Option<u8>,
        }

        let mut selector = jsonpath::selector_as::<Friend>(&json_obj);

        let json = selector("$..friends[0]").unwrap();

        let ret = vec!(
            Friend { name: "친구3".to_string(), age: Some(30) },
            Friend { name: "친구1".to_string(), age: Some(20) }
        );
        assert_eq!(json, ret);

        let json = selector("$..friends[1]").unwrap();

        let ret = vec!(
            Friend { name: "친구4".to_string(), age: None },
            Friend { name: "친구2".to_string(), age: Some(20) }
        );

        assert_eq!(json, ret);

        Ok(())
    }

    #[tokio::test]
    async fn find_cc_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusValidChangeToDetail",
            "maxRows": 10000
        }))?;

        let ret: SrvResp<serde_json::Value> = dele.srv("findCc", &values).await?;
        assert!(ret.is_ok());
        // println!("{}", ret.pretty_str()?);

        let data=ret.data.unwrap();
        let mut selector = jsonpath::selector(&data);

        let result=selector("$.result[*].statusId").unwrap();
        println!("{}", pretty(&result));

        Ok(())
    }

    #[tokio::test]
    async fn find_cc_wrapper_works() -> anyhow::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        let resp= find_cc(&dele, "Person", 5).await?;
        assert!(resp.is_ok());
        if let Some(rs)=resp.get_list("result"){
            for r in rs{
                println!("{}", pretty(r));
            }
        }

        Ok(())
    }
}

