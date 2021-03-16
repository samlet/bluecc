use crate::conn::establish_connection;
use diesel::prelude::*;
use serde_json::json;
use seed::new_snowflake_id;
use diesel::deserialize::Queryable;

mod schema {
    table! {
        use diesel::sql_types::*;
        use bigdecimal::BigDecimal;

        uom_type (uom_type_id) {
            parent_type_id -> Nullable<Int8>,
            has_table -> Nullable<Bool>,
            description -> Nullable<Varchar>,
            uom_type_id -> Int8,
        }
    }
}

use schema::uom_type;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_type_id)]
#[belongs_to(UomType, foreign_key = "parent_type_id")]
#[table_name = "uom_type"]
pub struct UomType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub uom_type_id: i64,
}

#[test]
fn it_works() -> anyhow::Result<()> {
    use schema::uom_type::dsl::*;

    let conn = establish_connection();
    let rs:Vec<UomType>=uom_type.order(uom_type_id.desc()).load(&conn)?;
    println!("get records {}", rs.len());
    Ok(())
}

