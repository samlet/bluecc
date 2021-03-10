#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum EntityTypes {

#[serde(rename_all = "camelCase")]
X509IssuerProvision{
    // keys
    cert_provision_id: Option<i64>, // id
    // fields
    common_name: Option<String>, // value
    organizational_unit: Option<String>, // value
    organization_name: Option<String>, // value
    city_locality: Option<String>, // value
    state_province: Option<String>, // value
    country: Option<String>, // value
    serial_number: Option<String> // value
},
        
#[serde(rename_all = "camelCase")]
UserLogin{
    // keys
    user_login_id: Option<i64>, // id-vlong
    // fields
    current_password: Option<String>, // long-varchar
    password_hint: Option<String>, // description
    is_system: Option<bool>, // indicator
    enabled: Option<bool>, // indicator
    has_logged_out: Option<bool>, // indicator
    require_password_change: Option<bool>, // indicator
    last_currency_uom: Option<i64>, // id
    last_locale: Option<String>, // very-short
    last_time_zone: Option<i64>, // id-long
    disabled_date_time: Option<chrono::NaiveDateTime>, // date-time
    successive_failed_logins: Option<i64>, // numeric
    external_auth_id: Option<i64>, // id-vlong
    user_ldap_dn: Option<i64>, // id-vlong
    disabled_by: Option<i64> // id-vlong
},
        
#[serde(rename_all = "camelCase")]
UserLoginPasswordHistory{
    // keys
    user_login_id: Option<i64>, // id-vlong
    from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    thru_date: Option<chrono::NaiveDateTime>, // date-time
    current_password: Option<String> // long-varchar
},
        
#[serde(rename_all = "camelCase")]
UserLoginHistory{
    // keys
    user_login_id: Option<i64>, // id-vlong
    from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    visit_id: Option<i64>, // id
    thru_date: Option<chrono::NaiveDateTime>, // date-time
    password_used: Option<String>, // long-varchar
    successful_login: Option<bool>, // indicator
    origin_user_login_id: Option<i64> // id-vlong
},
        
#[serde(rename_all = "camelCase")]
UserLoginSession{
    // keys
    user_login_id: Option<i64>, // id-vlong
    // fields
    saved_date: Option<chrono::NaiveDateTime>, // date-time
    session_data: Option<String> // very-long
},
        
#[serde(rename_all = "camelCase")]
SecurityGroup{
    // keys
    group_id: Option<i64>, // id
    // fields
    group_name: Option<String>, // value
    description: Option<String> // description
},
        
#[serde(rename_all = "camelCase")]
SecurityGroupPermission{
    // keys
    group_id: Option<i64>, // id
    permission_id: Option<i64>, // id-long
    from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    thru_date: Option<chrono::NaiveDateTime> // date-time
},
        
#[serde(rename_all = "camelCase")]
SecurityPermission{
    // keys
    permission_id: Option<i64>, // id-long
    // fields
    description: Option<String> // description
},
        
#[serde(rename_all = "camelCase")]
UserLoginSecurityGroup{
    // keys
    user_login_id: Option<i64>, // id-vlong
    group_id: Option<i64>, // id
    from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    thru_date: Option<chrono::NaiveDateTime> // date-time
},
        
#[serde(rename_all = "camelCase")]
ProtectedView{
    // keys
    group_id: Option<i64>, // id
    view_name_id: Option<i64>, // id-long
    // fields
    max_hits: Option<i64>, // numeric
    max_hits_duration: Option<i64>, // numeric
    tarpit_duration: Option<i64> // numeric
},
        
#[serde(rename_all = "camelCase")]
TarpittedLoginView{
    // keys
    view_name_id: Option<i64>, // id-long
    user_login_id: Option<i64>, // id-vlong
    // fields
    tarpit_release_date_time: Option<i64> // numeric
},
        }