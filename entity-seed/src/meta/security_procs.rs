use crate::establish_connection;
use crate::schema::{security_group, user_login};
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

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login"]
pub struct _UserLogin{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    // fields
    pub current_password: Option<String>, // long-varchar
    pub password_hint: Option<String>, // description
    pub is_system: Option<bool>, // indicator
    pub enabled: Option<bool>, // indicator
    pub has_logged_out: Option<bool>, // indicator
    pub require_password_change: Option<bool>, // indicator
    pub last_currency_uom: Option<i64>, // id
    pub last_locale: Option<String>, // very-short
    pub last_time_zone: Option<i64>, // id-long
    pub disabled_date_time: Option<chrono::NaiveDateTime>, // date-time
    pub successive_failed_logins: Option<i64>, // numeric
    pub external_auth_id: Option<i64>, // id-vlong
    pub user_ldap_dn: Option<i64>, // id-vlong
    pub disabled_by: Option<i64> // id-vlong
}

#[test]
fn user_login_works() -> anyhow::Result<()> {
    use crate::schema::user_login::dsl::*;

    let conn = establish_connection();
    let json = json!(
        {
          "userLoginId": new_snowflake_id(),
          "currentPassword": null,
          "passwordHint": null,
          "isSystem": true,
          "enabled": false,
          "hasLoggedOut": null,
          "requirePasswordChange": null,
          "lastCurrencyUom": null,
          "lastLocale": null,
          "lastTimeZone": null,
          "disabledDateTime": null,
          "successiveFailedLogins": null,
          "externalAuthId": null,
          "userLdapDn": null,
          "disabledBy": null
        }
    );
    let rec = serde_json::from_value::<_UserLogin>(json)?;
    diesel::insert_into(user_login).values(&rec).execute(&conn)?;
    Ok(())
}

