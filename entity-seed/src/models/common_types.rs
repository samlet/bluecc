use serde_derive::{Deserialize, Serialize};
// use crate::schema::common;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "data_source"]
pub struct DataSource{
    // keys
    pub data_source_id: Option<i64>, // id
    // fields
    pub data_source_type_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "data_source_type"]
pub struct DataSourceType{
    // keys
    pub data_source_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "email_template_setting"]
pub struct EmailTemplateSetting{
    // keys
    pub email_template_setting_id: Option<i64>, // id
    // fields
    pub email_type: Option<i64>, // id
    pub description: Option<String>, // description
    pub body_screen_location: Option<String>, // long-varchar
    pub xslfo_attach_screen_location: Option<String>, // long-varchar
    pub from_address: Option<String>, // email
    pub cc_address: Option<String>, // email
    pub bcc_address: Option<String>, // email
    pub subject: Option<String>, // comment
    pub content_type: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "enumeration"]
pub struct Enumeration{
    // keys
    pub enum_id: Option<i64>, // id
    // fields
    pub enum_type_id: Option<i64>, // id
    pub enum_code: Option<String>, // short-varchar
    pub sequence_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "enumeration_type"]
pub struct EnumerationType{
    // keys
    pub enum_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "country_capital"]
pub struct CountryCapital{
    // keys
    pub country_code: Option<i64>, // id
    // fields
    pub country_capital_name: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "country_code"]
pub struct CountryCode{
    // keys
    pub country_code_id: Option<i64>, // id
    // fields
    pub country_abbr: Option<String>, // short-varchar
    pub country_number: Option<String>, // short-varchar
    pub country_name: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "country_tele_code"]
pub struct CountryTeleCode{
    // keys
    pub country_code: Option<i64>, // id
    // fields
    pub tele_code: Option<String> // short-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "country_address_format"]
pub struct CountryAddressFormat{
    // keys
    pub geo_id: Option<i64>, // id
    // fields
    pub geo_assoc_type_id: Option<i64>, // id
    pub require_state_province_id: Option<i64>, // id
    pub require_postal_code: Option<bool>, // indicator
    pub postal_code_regex: Option<String>, // long-varchar
    pub has_postal_code_ext: Option<bool>, // indicator
    pub require_postal_code_ext: Option<bool>, // indicator
    pub address_format: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "geo"]
pub struct Geo{
    // keys
    pub geo_id: Option<i64>, // id
    // fields
    pub geo_type_id: Option<i64>, // id
    pub geo_name: Option<String>, // name
    pub geo_code: Option<String>, // short-varchar
    pub geo_sec_code: Option<String>, // short-varchar
    pub abbreviation: Option<String>, // short-varchar
    pub well_known_text: Option<String> // very-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "geo_assoc"]
pub struct GeoAssoc{
    // keys
    pub geo_id: Option<i64>, // id
    pub geo_id_to: Option<i64>, // id
    // fields
    pub geo_assoc_type_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "geo_assoc_type"]
pub struct GeoAssocType{
    // keys
    pub geo_assoc_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "geo_point"]
pub struct GeoPoint{
    // keys
    pub geo_point_id: Option<i64>, // id
    // fields
    pub geo_point_type_enum_id: Option<i64>, // id
    pub description: Option<String>, // description
    pub data_source_id: Option<i64>, // id
    pub latitude: Option<String>, // short-varchar
    pub longitude: Option<String>, // short-varchar
    pub elevation: Option<bigdecimal::BigDecimal>, // fixed-point
    pub elevation_uom_id: Option<i64>, // id
    pub information: Option<String> // comment
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "geo_type"]
pub struct GeoType{
    // keys
    pub geo_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "keyword_thesaurus"]
pub struct KeywordThesaurus{
    // keys
    pub entered_keyword: Option<String>, // long-varchar
    pub alternate_keyword: Option<String>, // long-varchar
    // fields
    pub relationship_enum_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "standard_language"]
pub struct StandardLanguage{
    // keys
    pub standard_language_id: Option<i64>, // id
    // fields
    pub lang_code_3t: Option<String>, // very-short
    pub lang_code_3b: Option<String>, // very-short
    pub lang_code_2: Option<String>, // very-short
    pub lang_name: Option<String>, // short-varchar
    pub lang_family: Option<String>, // short-varchar
    pub lang_charset: Option<String> // short-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "custom_method"]
pub struct CustomMethod{
    // keys
    pub custom_method_id: Option<i64>, // id
    // fields
    pub custom_method_type_id: Option<i64>, // id
    pub custom_method_name: Option<String>, // long-varchar
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "custom_method_type"]
pub struct CustomMethodType{
    // keys
    pub custom_method_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "note_data"]
pub struct NoteData{
    // keys
    pub note_id: Option<i64>, // id
    // fields
    pub note_name: Option<String>, // name
    pub note_info: Option<String>, // very-long
    pub note_date_time: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "custom_time_period"]
pub struct CustomTimePeriod{
    // keys
    pub custom_time_period_id: Option<i64>, // id
    // fields
    pub parent_period_id: Option<i64>, // id
    pub period_type_id: Option<i64>, // id
    pub period_num: Option<i64>, // numeric
    pub period_name: Option<String>, // name
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub is_closed: Option<bool> // indicator
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "period_type"]
pub struct PeriodType{
    // keys
    pub period_type_id: Option<i64>, // id
    // fields
    pub description: Option<String>, // description
    pub period_length: Option<i64>, // numeric
    pub uom_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "status_item"]
pub struct StatusItem{
    // keys
    pub status_id: Option<i64>, // id
    // fields
    pub status_type_id: Option<i64>, // id
    pub status_code: Option<String>, // short-varchar
    pub sequence_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "status_type"]
pub struct StatusType{
    // keys
    pub status_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "status_valid_change"]
pub struct StatusValidChange{
    // keys
    pub status_id: Option<i64>, // id
    pub status_id_to: Option<i64>, // id
    // fields
    pub condition_expression: Option<String>, // long-varchar
    pub transition_name: Option<String> // name
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "uom"]
pub struct Uom{
    // keys
    pub uom_id: Option<i64>, // id
    // fields
    pub uom_type_id: Option<i64>, // id
    pub abbreviation: Option<String>, // short-varchar
    pub numeric_code: Option<i64>, // numeric
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "uom_conversion"]
pub struct UomConversion{
    // keys
    pub uom_id: Option<i64>, // id
    pub uom_id_to: Option<i64>, // id
    // fields
    pub conversion_factor: Option<bigdecimal::BigDecimal>, // floating-point
    pub custom_method_id: Option<i64>, // id
    pub decimal_scale: Option<i64>, // numeric
    pub rounding_mode: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "uom_conversion_dated"]
pub struct UomConversionDated{
    // keys
    pub uom_id: Option<i64>, // id
    pub uom_id_to: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub conversion_factor: Option<bigdecimal::BigDecimal>, // floating-point
    pub custom_method_id: Option<i64>, // id
    pub decimal_scale: Option<i64>, // numeric
    pub rounding_mode: Option<i64>, // id
    pub purpose_enum_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "uom_group"]
pub struct UomGroup{
    // keys
    pub uom_group_id: Option<i64>, // id
    pub uom_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "uom_type"]
pub struct UomType{
    // keys
    pub uom_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_preference"]
pub struct UserPreference{
    // keys
    pub user_login_id: Option<i64>, // id-vlong
    pub user_pref_type_id: Option<i64>, // id-long
    // fields
    pub user_pref_group_type_id: Option<i64>, // id-long
    pub user_pref_value: Option<String>, // value
    pub user_pref_data_type: Option<i64> // id-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "user_pref_group_type"]
pub struct UserPrefGroupType{
    // keys
    pub user_pref_group_type_id: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "custom_screen"]
pub struct CustomScreen{
    // keys
    pub custom_screen_id: Option<i64>, // id
    // fields
    pub custom_screen_type_id: Option<i64>, // id
    pub custom_screen_name: Option<String>, // long-varchar
    pub custom_screen_location: Option<String>, // long-varchar
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "custom_screen_type"]
pub struct CustomScreenType{
    // keys
    pub custom_screen_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "visual_theme_set"]
pub struct VisualThemeSet{
    // keys
    pub visual_theme_set_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "visual_theme"]
pub struct VisualTheme{
    // keys
    pub visual_theme_id: Option<i64>, // id
    // fields
    pub visual_theme_set_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "visual_theme_resource"]
pub struct VisualThemeResource{
    // keys
    pub visual_theme_id: Option<i64>, // id
    pub resource_type_enum_id: Option<i64>, // id
    pub sequence_id: Option<i64>, // id
    // fields
    pub resource_value: Option<String> // value
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portal_portlet"]
pub struct PortalPortlet{
    // keys
    pub portal_portlet_id: Option<i64>, // id
    // fields
    pub portlet_name: Option<String>, // name
    pub screen_name: Option<String>, // long-varchar
    pub screen_location: Option<String>, // long-varchar
    pub edit_form_name: Option<String>, // long-varchar
    pub edit_form_location: Option<String>, // long-varchar
    pub description: Option<String>, // description
    pub screenshot: Option<String>, // url
    pub security_service_name: Option<String>, // long-varchar
    pub security_main_action: Option<String> // short-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portlet_category"]
pub struct PortletCategory{
    // keys
    pub portlet_category_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portlet_portlet_category"]
pub struct PortletPortletCategory{
    // keys
    pub portal_portlet_id: Option<i64>, // id
    pub portlet_category_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portal_page"]
pub struct PortalPage{
    // keys
    pub portal_page_id: Option<i64>, // id
    // fields
    pub portal_page_name: Option<String>, // name
    pub description: Option<String>, // description
    pub owner_user_login_id: Option<i64>, // id-vlong
    pub original_portal_page_id: Option<i64>, // id
    pub parent_portal_page_id: Option<i64>, // id
    pub sequence_num: Option<i64>, // numeric
    pub security_group_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portal_page_column"]
pub struct PortalPageColumn{
    // keys
    pub portal_page_id: Option<i64>, // id
    pub column_seq_id: Option<i64>, // id
    // fields
    pub column_width_pixels: Option<i64>, // numeric
    pub column_width_percentage: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portal_page_portlet"]
pub struct PortalPagePortlet{
    // keys
    pub portal_page_id: Option<i64>, // id
    pub portal_portlet_id: Option<i64>, // id
    pub portlet_seq_id: Option<i64>, // id
    // fields
    pub column_seq_id: Option<i64>, // id
    pub sequence_num: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "portlet_attribute"]
pub struct PortletAttribute{
    // keys
    pub portal_page_id: Option<i64>, // id
    pub portal_portlet_id: Option<i64>, // id
    pub portlet_seq_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String>, // description
    pub attr_type: Option<String> // value
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "system_property"]
pub struct SystemProperty{
    // keys
    pub system_resource_id: Option<i64>, // id-long
    pub system_property_id: Option<i64>, // id-long
    // fields
    pub system_property_value: Option<String>, // value
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "telecom_method_type"]
pub struct TelecomMethodType{
    // keys
    pub telecom_method_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "telecom_gateway_config"]
pub struct TelecomGatewayConfig{
    // keys
    pub telecom_gateway_config_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

