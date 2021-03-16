use serde_derive::{Deserialize, Serialize};
// use crate::schema::security;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(cert_provision_id)]
#[table_name = "x509_issuer_provision"]
pub struct X509IssuerProvision{
    // fields
    pub common_name: Option<String>,
    pub organizational_unit: Option<String>,
    pub organization_name: Option<String>,
    pub city_locality: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub serial_number: Option<String>,
    // keys
    pub cert_provision_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(user_login_id)]
#[table_name = "user_login"]
pub struct UserLogin{
    // fields
    pub current_password: Option<String>,
    pub password_hint: Option<String>,
    pub is_system: Option<bool>,
    pub enabled: Option<bool>,
    pub has_logged_out: Option<bool>,
    pub require_password_change: Option<bool>,
    pub last_currency_uom: Option<i64>,
    pub last_locale: Option<String>,
    pub last_time_zone: Option<i64>,
    pub disabled_date_time: Option<chrono::NaiveDateTime>,
    pub successive_failed_logins: Option<i64>,
    pub external_auth_id: Option<i64>,
    pub user_ldap_dn: Option<i64>,
    pub disabled_by: Option<i64>,
    // keys
    pub user_login_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id, from_date)]
#[belongs_to(UserLogin, foreign_key = "user_login_id")]
#[table_name = "user_login_password_history"]
pub struct UserLoginPasswordHistory{
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>,
    pub current_password: Option<String>,
    // keys
    pub user_login_id: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id, from_date)]
#[belongs_to(UserLogin, foreign_key = "user_login_id")]
#[table_name = "user_login_history"]
pub struct UserLoginHistory{
    // fields
    pub visit_id: Option<i64>,
    pub thru_date: Option<chrono::NaiveDateTime>,
    pub password_used: Option<String>,
    pub successful_login: Option<bool>,
    pub origin_user_login_id: Option<i64>,
    // keys
    pub user_login_id: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id)]
#[belongs_to(UserLogin, foreign_key = "user_login_id")]
#[table_name = "user_login_session"]
pub struct UserLoginSession{
    // fields
    pub saved_date: Option<chrono::NaiveDateTime>,
    pub session_data: Option<String>,
    // keys
    pub user_login_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(group_id)]
#[table_name = "security_group"]
pub struct SecurityGroup{
    // fields
    pub group_name: Option<String>,
    pub description: Option<String>,
    // keys
    pub group_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(group_id, permission_id, from_date)]
#[belongs_to(SecurityGroup, foreign_key = "group_id")]
#[belongs_to(SecurityPermission, foreign_key = "permission_id")]
#[table_name = "security_group_permission"]
pub struct SecurityGroupPermission{
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>,
    // keys
    pub group_id: i64,
    pub permission_id: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(permission_id)]
#[table_name = "security_permission"]
pub struct SecurityPermission{
    // fields
    pub description: Option<String>,
    // keys
    pub permission_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id, group_id, from_date)]
#[belongs_to(UserLogin, foreign_key = "user_login_id")]
#[belongs_to(SecurityGroup, foreign_key = "group_id")]
#[table_name = "user_login_security_group"]
pub struct UserLoginSecurityGroup{
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>,
    // keys
    pub user_login_id: i64,
    pub group_id: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(group_id, view_name_id)]
#[belongs_to(SecurityGroup, foreign_key = "group_id")]
#[table_name = "protected_view"]
pub struct ProtectedView{
    // fields
    pub max_hits: Option<i64>,
    pub max_hits_duration: Option<i64>,
    pub tarpit_duration: Option<i64>,
    // keys
    pub group_id: i64,
    pub view_name_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(view_name_id, user_login_id)]
#[table_name = "tarpitted_login_view"]
pub struct TarpittedLoginView{
    // fields
    pub tarpit_release_date_time: Option<i64>,
    // keys
    pub view_name_id: i64,
    pub user_login_id: i64,
}

