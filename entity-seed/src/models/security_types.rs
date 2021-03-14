use serde_derive::{Deserialize, Serialize};
// use crate::schema::security;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "x509_issuer_provision"]
pub struct X509IssuerProvision{
    // keys
    pub cert_provision_id: Option<i64>, // id
    // fields
    pub common_name: Option<String>, // value
    pub organizational_unit: Option<String>, // value
    pub organization_name: Option<String>, // value
    pub city_locality: Option<String>, // value
    pub state_province: Option<String>, // value
    pub country: Option<String>, // value
    pub serial_number: Option<String> // value
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login"]
pub struct UserLogin{
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

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login_password_history"]
pub struct UserLoginPasswordHistory{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub current_password: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login_history"]
pub struct UserLoginHistory{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub visit_id: Option<i64>, // id
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub password_used: Option<String>, // long-varchar
    pub successful_login: Option<bool>, // indicator
    pub origin_user_login_id: Option<i64> // id-vlong
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login_session"]
pub struct UserLoginSession{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    // fields
    pub saved_date: Option<chrono::NaiveDateTime>, // date-time
    pub session_data: Option<String> // very-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "security_group"]
pub struct SecurityGroup{
    // keys
    pub group_id: Option<i64>, // id
    // fields
    pub group_name: Option<String>, // value
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "security_group_permission"]
pub struct SecurityGroupPermission{
    // keys
    pub group_id: Option<i64>, // id
    pub permission_id: Option<i64>, // id-long
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "security_permission"]
pub struct SecurityPermission{
    // keys
    pub permission_id: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_login_security_group"]
pub struct UserLoginSecurityGroup{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    pub group_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "protected_view"]
pub struct ProtectedView{
    // keys
    pub group_id: Option<i64>, // id
    pub view_name_id: Option<i64>, // id-long
    // fields
    pub max_hits: Option<i64>, // numeric
    pub max_hits_duration: Option<i64>, // numeric
    pub tarpit_duration: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "tarpitted_login_view"]
pub struct TarpittedLoginView{
    // keys
    pub view_name_id: Option<i64>, // id-long
    pub user_login_id: Option<i64>, // id-vlong
    // fields
    pub tarpit_release_date_time: Option<i64> // numeric
}

