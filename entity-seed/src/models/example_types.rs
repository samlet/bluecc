use serde_derive::{Deserialize, Serialize};
// use crate::schema::example;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example"]
pub struct Example{
    // keys
    pub example_id: Option<i64>, // id
    // fields
    pub example_type_id: Option<i64>, // id
    pub status_id: Option<i64>, // id
    pub example_name: Option<String>, // name
    pub description: Option<String>, // description
    pub long_description: Option<String>, // very-long
    pub comments: Option<String>, // comment
    pub example_size: Option<i64>, // numeric
    pub example_date: Option<chrono::NaiveDateTime>, // date-time
    pub another_date: Option<chrono::NaiveDateTime>, // date-time
    pub another_text: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_item"]
pub struct ExampleItem{
    // keys
    pub example_id: Option<i64>, // id
    pub example_item_seq_id: Option<i64>, // id
    // fields
    pub description: Option<String>, // description
    pub amount: Option<f64>, // floating-point
    pub amount_uom_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_status"]
pub struct ExampleStatus{
    // keys
    pub example_id: Option<i64>, // id
    pub status_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub status_end_date: Option<chrono::NaiveDateTime>, // date-time
    pub change_by_user_login_id: Option<i64>, // id-vlong
    pub status_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_type"]
pub struct ExampleType{
    // keys
    pub example_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_feature"]
pub struct ExampleFeature{
    // keys
    pub example_feature_id: Option<i64>, // id
    // fields
    pub feature_source_enum_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_feature_appl"]
pub struct ExampleFeatureAppl{
    // keys
    pub example_id: Option<i64>, // id
    pub example_feature_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub example_feature_appl_type_id: Option<i64>, // id
    pub sequence_num: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "example_feature_appl_type"]
pub struct ExampleFeatureApplType{
    // keys
    pub example_feature_appl_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub description: Option<String> // description
}

