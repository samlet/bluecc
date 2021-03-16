use serde_derive::{Deserialize, Serialize};
// use crate::schema::example;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id)]
#[belongs_to(ExampleType, foreign_key = "example_type_id")]
#[table_name = "example"]
pub struct Example{
    // fields
    pub example_type_id: Option<i64>,
    pub status_id: Option<i64>,
    pub example_name: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub comments: Option<String>,
    pub example_size: Option<i64>,
    pub example_date: Option<chrono::NaiveDateTime>,
    pub another_date: Option<chrono::NaiveDateTime>,
    pub another_text: Option<String>,
    // keys
    pub example_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, example_item_seq_id)]
#[belongs_to(Example, foreign_key = "example_id")]
#[table_name = "example_item"]
pub struct ExampleItem{
    // fields
    pub description: Option<String>,
    pub amount: Option<bigdecimal::BigDecimal>,
    pub amount_uom_id: Option<i64>,
    // keys
    pub example_id: i64,
    pub example_item_seq_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, status_date)]
#[belongs_to(Example, foreign_key = "example_id")]
#[table_name = "example_status"]
pub struct ExampleStatus{
    // fields
    pub status_end_date: Option<chrono::NaiveDateTime>,
    pub change_by_user_login_id: Option<i64>,
    pub status_id: Option<i64>,
    // keys
    pub example_id: i64,
    pub status_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_type_id)]
#[belongs_to(ExampleType, foreign_key = "parent_type_id")]
#[table_name = "example_type"]
pub struct ExampleType{
    // fields
    pub parent_type_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub example_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(example_feature_id)]
#[table_name = "example_feature"]
pub struct ExampleFeature{
    // fields
    pub feature_source_enum_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub example_feature_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, example_feature_id, from_date)]
#[belongs_to(Example, foreign_key = "example_id")]
#[belongs_to(ExampleFeature, foreign_key = "example_feature_id")]
#[belongs_to(ExampleFeatureApplType, foreign_key = "example_feature_appl_type_id")]
#[table_name = "example_feature_appl"]
pub struct ExampleFeatureAppl{
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>,
    pub example_feature_appl_type_id: Option<i64>,
    pub sequence_num: Option<i64>,
    // keys
    pub example_id: i64,
    pub example_feature_id: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_feature_appl_type_id)]
#[belongs_to(ExampleFeatureApplType, foreign_key = "parent_type_id")]
#[table_name = "example_feature_appl_type"]
pub struct ExampleFeatureApplType{
    // fields
    pub parent_type_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub example_feature_appl_type_id: i64,
}

