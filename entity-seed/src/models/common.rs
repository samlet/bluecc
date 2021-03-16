use serde_derive::{Deserialize, Serialize};
// use crate::schema::common;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(data_source_id)]
#[belongs_to(DataSourceType, foreign_key = "data_source_type_id")]
#[table_name = "data_source"]
pub struct DataSource{
    // fields
    pub data_source_type_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub data_source_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(data_source_type_id)]
#[table_name = "data_source_type"]
pub struct DataSourceType{
    // fields
    pub description: Option<String>,
    // keys
    pub data_source_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(email_template_setting_id)]
#[belongs_to(Enumeration, foreign_key = "email_type")]
#[table_name = "email_template_setting"]
pub struct EmailTemplateSetting{
    // fields
    pub email_type: Option<i64>,
    pub description: Option<String>,
    pub body_screen_location: Option<String>,
    pub xslfo_attach_screen_location: Option<String>,
    pub from_address: Option<String>,
    pub cc_address: Option<String>,
    pub bcc_address: Option<String>,
    pub subject: Option<String>,
    pub content_type: Option<String>,
    // keys
    pub email_template_setting_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(enum_id)]
#[belongs_to(EnumerationType, foreign_key = "enum_type_id")]
#[table_name = "enumeration"]
pub struct Enumeration{
    // fields
    pub enum_type_id: Option<i64>,
    pub enum_code: Option<String>,
    pub sequence_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub enum_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(enum_type_id)]
#[belongs_to(EnumerationType, foreign_key = "parent_type_id")]
#[table_name = "enumeration_type"]
pub struct EnumerationType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub enum_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(country_code)]
#[belongs_to(CountryCode, foreign_key = "country_code")]
#[table_name = "country_capital"]
pub struct CountryCapital{
    // fields
    pub country_capital_name: Option<String>,
    // keys
    pub country_code: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(country_code_id)]
#[table_name = "country_code"]
pub struct CountryCode{
    // fields
    pub country_abbr: Option<String>,
    pub country_number: Option<String>,
    pub country_name: Option<String>,
    // keys
    pub country_code_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(country_code)]
#[belongs_to(CountryCode, foreign_key = "country_code")]
#[table_name = "country_tele_code"]
pub struct CountryTeleCode{
    // fields
    pub tele_code: Option<String>,
    // keys
    pub country_code: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id)]
#[belongs_to(Geo, foreign_key = "geo_id")]
#[belongs_to(GeoAssocType, foreign_key = "geo_assoc_type_id")]
#[table_name = "country_address_format"]
pub struct CountryAddressFormat{
    // fields
    pub geo_assoc_type_id: Option<i64>,
    pub require_state_province_id: Option<i64>,
    pub require_postal_code: Option<bool>,
    pub postal_code_regex: Option<String>,
    pub has_postal_code_ext: Option<bool>,
    pub require_postal_code_ext: Option<bool>,
    pub address_format: Option<String>,
    // keys
    pub geo_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id)]
#[belongs_to(GeoType, foreign_key = "geo_type_id")]
#[table_name = "geo"]
pub struct Geo{
    // fields
    pub geo_type_id: Option<i64>,
    pub geo_name: Option<String>,
    pub geo_code: Option<String>,
    pub geo_sec_code: Option<String>,
    pub abbreviation: Option<String>,
    pub well_known_text: Option<String>,
    // keys
    pub geo_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id, geo_id_to)]
#[belongs_to(Geo, foreign_key = "geo_id")]
#[belongs_to(GeoAssocType, foreign_key = "geo_assoc_type_id")]
#[table_name = "geo_assoc"]
pub struct GeoAssoc{
    // fields
    pub geo_assoc_type_id: Option<i64>,
    // keys
    pub geo_id: i64,
    pub geo_id_to: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(geo_assoc_type_id)]
#[table_name = "geo_assoc_type"]
pub struct GeoAssocType{
    // fields
    pub description: Option<String>,
    // keys
    pub geo_assoc_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_point_id)]
#[belongs_to(DataSource, foreign_key = "data_source_id")]
#[belongs_to(Enumeration, foreign_key = "geo_point_type_enum_id")]
#[belongs_to(Uom, foreign_key = "elevation_uom_id")]
#[table_name = "geo_point"]
pub struct GeoPoint{
    // fields
    pub geo_point_type_enum_id: Option<i64>,
    pub description: Option<String>,
    pub data_source_id: Option<i64>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub elevation: Option<bigdecimal::BigDecimal>,
    pub elevation_uom_id: Option<i64>,
    pub information: Option<String>,
    // keys
    pub geo_point_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_type_id)]
#[belongs_to(GeoType, foreign_key = "parent_type_id")]
#[table_name = "geo_type"]
pub struct GeoType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub geo_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(entered_keyword, alternate_keyword)]
#[belongs_to(Enumeration, foreign_key = "relationship_enum_id")]
#[table_name = "keyword_thesaurus"]
pub struct KeywordThesaurus{
    // fields
    pub relationship_enum_id: Option<i64>,
    // keys
    pub entered_keyword: Option<String>,
    pub alternate_keyword: Option<String>,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(standard_language_id)]
#[table_name = "standard_language"]
pub struct StandardLanguage{
    // fields
    pub lang_code_3t: Option<String>,
    pub lang_code_3b: Option<String>,
    pub lang_code_2: Option<String>,
    pub lang_name: Option<String>,
    pub lang_family: Option<String>,
    pub lang_charset: Option<String>,
    // keys
    pub standard_language_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_method_id)]
#[belongs_to(CustomMethodType, foreign_key = "custom_method_type_id")]
#[table_name = "custom_method"]
pub struct CustomMethod{
    // fields
    pub custom_method_type_id: Option<i64>,
    pub custom_method_name: Option<String>,
    pub description: Option<String>,
    // keys
    pub custom_method_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_method_type_id)]
#[belongs_to(CustomMethodType, foreign_key = "parent_type_id")]
#[table_name = "custom_method_type"]
pub struct CustomMethodType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub custom_method_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(note_id)]
#[table_name = "note_data"]
pub struct NoteData{
    // fields
    pub note_name: Option<String>,
    pub note_info: Option<String>,
    pub note_date_time: Option<chrono::NaiveDateTime>,
    // keys
    pub note_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_time_period_id)]
#[belongs_to(CustomTimePeriod, foreign_key = "parent_period_id")]
#[belongs_to(PeriodType, foreign_key = "period_type_id")]
#[table_name = "custom_time_period"]
pub struct CustomTimePeriod{
    // fields
    pub parent_period_id: Option<i64>,
    pub period_type_id: Option<i64>,
    pub period_num: Option<i64>,
    pub period_name: Option<String>,
    pub from_date: Option<chrono::NaiveDateTime>,
    pub thru_date: Option<chrono::NaiveDateTime>,
    pub is_closed: Option<bool>,
    // keys
    pub custom_time_period_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(period_type_id)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[table_name = "period_type"]
pub struct PeriodType{
    // fields
    pub description: Option<String>,
    pub period_length: Option<i64>,
    pub uom_id: Option<i64>,
    // keys
    pub period_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_id)]
#[belongs_to(StatusType, foreign_key = "status_type_id")]
#[table_name = "status_item"]
pub struct StatusItem{
    // fields
    pub status_type_id: Option<i64>,
    pub status_code: Option<String>,
    pub sequence_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub status_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_type_id)]
#[belongs_to(StatusType, foreign_key = "parent_type_id")]
#[table_name = "status_type"]
pub struct StatusType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub status_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_id, status_id_to)]
#[belongs_to(StatusItem, foreign_key = "status_id")]
#[table_name = "status_valid_change"]
pub struct StatusValidChange{
    // fields
    pub condition_expression: Option<String>,
    pub transition_name: Option<String>,
    // keys
    pub status_id: i64,
    pub status_id_to: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id)]
#[belongs_to(UomType, foreign_key = "uom_type_id")]
#[table_name = "uom"]
pub struct Uom{
    // fields
    pub uom_type_id: Option<i64>,
    pub abbreviation: Option<String>,
    pub numeric_code: Option<i64>,
    pub description: Option<String>,
    // keys
    pub uom_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id, uom_id_to)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[belongs_to(CustomMethod, foreign_key = "custom_method_id")]
#[table_name = "uom_conversion"]
pub struct UomConversion{
    // fields
    pub conversion_factor: Option<bigdecimal::BigDecimal>,
    pub custom_method_id: Option<i64>,
    pub decimal_scale: Option<i64>,
    pub rounding_mode: Option<i64>,
    // keys
    pub uom_id: i64,
    pub uom_id_to: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id, uom_id_to, from_date)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[belongs_to(CustomMethod, foreign_key = "custom_method_id")]
#[belongs_to(Enumeration, foreign_key = "purpose_enum_id")]
#[table_name = "uom_conversion_dated"]
pub struct UomConversionDated{
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>,
    pub conversion_factor: Option<bigdecimal::BigDecimal>,
    pub custom_method_id: Option<i64>,
    pub decimal_scale: Option<i64>,
    pub rounding_mode: Option<i64>,
    pub purpose_enum_id: Option<i64>,
    // keys
    pub uom_id: i64,
    pub uom_id_to: i64,
    pub from_date: chrono::NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_group_id, uom_id)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[table_name = "uom_group"]
pub struct UomGroup{
    // fields
    // keys
    pub uom_group_id: i64,
    pub uom_id: i64,
}

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

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id, user_pref_type_id)]
#[belongs_to(UserPrefGroupType, foreign_key = "user_pref_group_type_id")]
#[table_name = "user_preference"]
pub struct UserPreference{
    // fields
    pub user_pref_group_type_id: Option<i64>,
    pub user_pref_value: Option<String>,
    pub user_pref_data_type: Option<i64>,
    // keys
    pub user_login_id: i64,
    pub user_pref_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(user_pref_group_type_id)]
#[table_name = "user_pref_group_type"]
pub struct UserPrefGroupType{
    // fields
    pub description: Option<String>,
    // keys
    pub user_pref_group_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_screen_id)]
#[belongs_to(CustomScreenType, foreign_key = "custom_screen_type_id")]
#[table_name = "custom_screen"]
pub struct CustomScreen{
    // fields
    pub custom_screen_type_id: Option<i64>,
    pub custom_screen_name: Option<String>,
    pub custom_screen_location: Option<String>,
    pub description: Option<String>,
    // keys
    pub custom_screen_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(custom_screen_type_id)]
#[table_name = "custom_screen_type"]
pub struct CustomScreenType{
    // fields
    pub parent_type_id: Option<i64>,
    pub has_table: Option<bool>,
    pub description: Option<String>,
    // keys
    pub custom_screen_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(visual_theme_set_id)]
#[table_name = "visual_theme_set"]
pub struct VisualThemeSet{
    // fields
    pub description: Option<String>,
    // keys
    pub visual_theme_set_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(visual_theme_id)]
#[belongs_to(VisualThemeSet, foreign_key = "visual_theme_set_id")]
#[table_name = "visual_theme"]
pub struct VisualTheme{
    // fields
    pub visual_theme_set_id: Option<i64>,
    pub description: Option<String>,
    // keys
    pub visual_theme_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(visual_theme_id, resource_type_enum_id, sequence_id)]
#[belongs_to(VisualTheme, foreign_key = "visual_theme_id")]
#[belongs_to(Enumeration, foreign_key = "resource_type_enum_id")]
#[table_name = "visual_theme_resource"]
pub struct VisualThemeResource{
    // fields
    pub resource_value: Option<String>,
    // keys
    pub visual_theme_id: i64,
    pub resource_type_enum_id: i64,
    pub sequence_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(portal_portlet_id)]
#[table_name = "portal_portlet"]
pub struct PortalPortlet{
    // fields
    pub portlet_name: Option<String>,
    pub screen_name: Option<String>,
    pub screen_location: Option<String>,
    pub edit_form_name: Option<String>,
    pub edit_form_location: Option<String>,
    pub description: Option<String>,
    pub screenshot: Option<String>,
    pub security_service_name: Option<String>,
    pub security_main_action: Option<String>,
    // keys
    pub portal_portlet_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(portlet_category_id)]
#[table_name = "portlet_category"]
pub struct PortletCategory{
    // fields
    pub description: Option<String>,
    // keys
    pub portlet_category_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_portlet_id, portlet_category_id)]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[belongs_to(PortletCategory, foreign_key = "portlet_category_id")]
#[table_name = "portlet_portlet_category"]
pub struct PortletPortletCategory{
    // fields
    // keys
    pub portal_portlet_id: i64,
    pub portlet_category_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id)]
#[belongs_to(PortalPage, foreign_key = "parent_portal_page_id")]
#[table_name = "portal_page"]
pub struct PortalPage{
    // fields
    pub portal_page_name: Option<String>,
    pub description: Option<String>,
    pub owner_user_login_id: Option<i64>,
    pub original_portal_page_id: Option<i64>,
    pub parent_portal_page_id: Option<i64>,
    pub sequence_num: Option<i64>,
    pub security_group_id: Option<i64>,
    // keys
    pub portal_page_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, column_seq_id)]
#[belongs_to(PortalPage, foreign_key = "portal_page_id")]
#[table_name = "portal_page_column"]
pub struct PortalPageColumn{
    // fields
    pub column_width_pixels: Option<i64>,
    pub column_width_percentage: Option<i64>,
    // keys
    pub portal_page_id: i64,
    pub column_seq_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, portal_portlet_id, portlet_seq_id)]
#[belongs_to(PortalPage, foreign_key = "portal_page_id")]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[table_name = "portal_page_portlet"]
pub struct PortalPagePortlet{
    // fields
    pub column_seq_id: Option<i64>,
    pub sequence_num: Option<i64>,
    // keys
    pub portal_page_id: i64,
    pub portal_portlet_id: i64,
    pub portlet_seq_id: i64,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, portal_portlet_id, portlet_seq_id, attr_name)]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[table_name = "portlet_attribute"]
pub struct PortletAttribute{
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>,
    pub attr_type: Option<String>,
    // keys
    pub portal_page_id: i64,
    pub portal_portlet_id: i64,
    pub portlet_seq_id: i64,
    pub attr_name: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(system_resource_id, system_property_id)]
#[table_name = "system_property"]
pub struct SystemProperty{
    // fields
    pub system_property_value: Option<String>,
    pub description: Option<String>,
    // keys
    pub system_resource_id: i64,
    pub system_property_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(telecom_method_type_id)]
#[table_name = "telecom_method_type"]
pub struct TelecomMethodType{
    // fields
    pub description: Option<String>,
    // keys
    pub telecom_method_type_id: i64,
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(telecom_gateway_config_id)]
#[table_name = "telecom_gateway_config"]
pub struct TelecomGatewayConfig{
    // fields
    pub description: Option<String>,
    // keys
    pub telecom_gateway_config_id: i64,
}

