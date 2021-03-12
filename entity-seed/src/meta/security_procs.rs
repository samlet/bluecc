use crate::establish_connection;
use crate::schema::security_group;
use diesel::prelude::*;
use serde_json::json;
use crate::new_snowflake_id;

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "security_group"]
pub struct _SecurityGroup{
    // keys
    pub group_id: Option<i64>, // id
    // fields
    pub group_name: Option<String>, // value
    pub description: Option<String> // description
}

#[test]
fn store_works() -> anyhow::Result<()> {
    use crate::schema::security_group::dsl::*;

    let conn = establish_connection();
    let json = r#"
        {
          "groupId": 212118821551607808,
          "groupName": "Full Admin",
          "description": "Full Admin group, has all general permissions."
        }
    "#;
    let rec = serde_json::from_str::<_SecurityGroup>(json)?;
    diesel::insert_into(security_group).values(&rec).execute(&conn)?;
    Ok(())
}

#[test]
fn store_json_works() -> anyhow::Result<()> {
    // use crate::models::security_types::SecurityGroup;
    use crate::schema::security_group::dsl::*;

    let conn = establish_connection();
    let json = json!(
        {
          // "groupId": 212118821551607808_i64,
          "groupId": new_snowflake_id(),
          "groupName": "Full Admin",
          "description": "Full Admin group, has all general permissions."
        }
    );
    let rec = serde_json::from_value::<_SecurityGroup>(json)?;
    diesel::insert_into(security_group).values(&rec).execute(&conn)?;
    Ok(())
}

