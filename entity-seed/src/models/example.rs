use serde_derive::{Deserialize, Serialize};
// use crate::schema::example;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id)]
#[belongs_to(ExampleType, foreign_key = "example_type_id")]
#[table_name = "example"]
pub struct Example{
    // keys
    pub example_id: i64,
    // fields
    pub example_type_id: i64,
    pub status_id: i64,
    pub example_name: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub comments: Option<String>,
    pub example_size: i64,
    pub example_date: chrono::NaiveDateTime,
    pub another_date: chrono::NaiveDateTime,
    pub another_text: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, example_item_seq_id)]
#[belongs_to(Example, foreign_key = "example_id")]
#[table_name = "example_item"]
pub struct ExampleItem{
    // keys
    pub example_id: i64,
    pub example_item_seq_id: i64,
    // fields
    pub description: Option<String>,
    pub amount: f64,
    pub amount_uom_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, status_date)]
#[belongs_to(Example, foreign_key = "example_id")]
#[table_name = "example_status"]
pub struct ExampleStatus{
    // keys
    pub example_id: i64,
    pub status_date: chrono::NaiveDateTime,
    // fields
    pub status_end_date: chrono::NaiveDateTime,
    pub change_by_user_login_id: i64,
    pub status_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_type_id)]
#[belongs_to(ExampleType, foreign_key = "parent_type_id")]
#[table_name = "example_type"]
pub struct ExampleType{
    // keys
    pub example_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(example_feature_id)]
#[table_name = "example_feature"]
pub struct ExampleFeature{
    // keys
    pub example_feature_id: i64,
    // fields
    pub feature_source_enum_id: i64,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_id, example_feature_id, from_date)]
#[belongs_to(Example, foreign_key = "example_id")]
#[belongs_to(ExampleFeature, foreign_key = "example_feature_id")]
#[belongs_to(ExampleFeatureApplType, foreign_key = "example_feature_appl_type_id")]
#[table_name = "example_feature_appl"]
pub struct ExampleFeatureAppl{
    // keys
    pub example_id: i64,
    pub example_feature_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub example_feature_appl_type_id: i64,
    pub sequence_num: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(example_feature_appl_type_id)]
#[belongs_to(ExampleFeatureApplType, foreign_key = "parent_type_id")]
#[table_name = "example_feature_appl_type"]
pub struct ExampleFeatureApplType{
    // keys
    pub example_feature_appl_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub description: Option<String>
}

