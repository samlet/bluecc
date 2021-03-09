use serde_derive::{Deserialize, Serialize};
use crate::schema::{{table}};
use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(data_source_id)]
#[belongs_to(DataSourceType, foreign_key = "data_source_type_id")]
#[table_name = "data_source"]
pub struct DataSource{
    // keys
    pub data_source_id: i32,
    // fields
    pub data_source_type_id: i32,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(data_source_type_id)]
#[table_name = "data_source_type"]
pub struct DataSourceType{
    // keys
    pub data_source_type_id: i32,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(email_template_setting_id)]
#[belongs_to(Enumeration, foreign_key = "email_type")]
#[table_name = "email_template_setting"]
pub struct EmailTemplateSetting{
    // keys
    pub email_template_setting_id: i32,
    // fields
    pub email_type: i32,
    pub description: Option<String>,
    pub body_screen_location: Option<String>,
    pub xslfo_attach_screen_location: Option<String>,
    pub from_address: Option<String>,
    pub cc_address: Option<String>,
    pub bcc_address: Option<String>,
    pub subject: Option<String>,
    pub content_type: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(enum_id)]
#[belongs_to(EnumerationType, foreign_key = "enum_type_id")]
#[table_name = "enumeration"]
pub struct Enumeration{
    // keys
    pub enum_id: i32,
    // fields
    pub enum_type_id: i32,
    pub enum_code: Option<String>,
    pub sequence_id: i32,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(enum_type_id)]
#[belongs_to(EnumerationType, foreign_key = "parent_type_id")]
#[table_name = "enumeration_type"]
pub struct EnumerationType{
    // keys
    pub enum_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(country_code)]
#[belongs_to(CountryCode, foreign_key = "country_code")]
#[table_name = "country_capital"]
pub struct CountryCapital{
    // keys
    pub country_code: i32,
    // fields
    pub country_capital: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(country_code)]
#[table_name = "country_code"]
pub struct CountryCode{
    // keys
    pub country_code: i32,
    // fields
    pub country_abbr: Option<String>,
    pub country_number: Option<String>,
    pub country_name: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(country_code)]
#[belongs_to(CountryCode, foreign_key = "country_code")]
#[table_name = "country_tele_code"]
pub struct CountryTeleCode{
    // keys
    pub country_code: i32,
    // fields
    pub tele_code: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id)]
#[belongs_to(Geo, foreign_key = "geo_id")]
#[belongs_to(GeoAssocType, foreign_key = "geo_assoc_type_id")]
#[table_name = "country_address_format"]
pub struct CountryAddressFormat{
    // keys
    pub geo_id: i32,
    // fields
    pub geo_assoc_type_id: i32,
    pub require_state_province_id: i32,
    pub require_postal_code: bool,
    pub postal_code_regex: Option<String>,
    pub has_postal_code_ext: bool,
    pub require_postal_code_ext: bool,
    pub address_format: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id)]
#[belongs_to(GeoType, foreign_key = "geo_type_id")]
#[table_name = "geo"]
pub struct Geo{
    // keys
    pub geo_id: i32,
    // fields
    pub geo_type_id: i32,
    pub geo_name: Option<String>,
    pub geo_code: Option<String>,
    pub geo_sec_code: Option<String>,
    pub abbreviation: Option<String>,
    pub well_known_text: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_id, geo_id_to)]
#[belongs_to(Geo, foreign_key = "geo_id")]
#[belongs_to(Geo, foreign_key = "geo_id_to")]
#[belongs_to(GeoAssocType, foreign_key = "geo_assoc_type_id")]
#[table_name = "geo_assoc"]
pub struct GeoAssoc{
    // keys
    pub geo_id: i32,
    pub geo_id_to: i32,
    // fields
    pub geo_assoc_type_id: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_assoc_type_id)]
#[table_name = "geo_assoc_type"]
pub struct GeoAssocType{
    // keys
    pub geo_assoc_type_id: i32,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_point_id)]
#[belongs_to(DataSource, foreign_key = "data_source_id")]
#[belongs_to(Enumeration, foreign_key = "geo_point_type_enum_id")]
#[belongs_to(Uom, foreign_key = "elevation_uom_id")]
#[table_name = "geo_point"]
pub struct GeoPoint{
    // keys
    pub geo_point_id: i32,
    // fields
    pub geo_point_type_enum_id: i32,
    pub description: Option<String>,
    pub data_source_id: i32,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub elevation: **UNK(fixed-point)**,
    pub elevation_uom_id: i32,
    pub information: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(geo_type_id)]
#[belongs_to(GeoType, foreign_key = "parent_type_id")]
#[table_name = "geo_type"]
pub struct GeoType{
    // keys
    pub geo_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(entered_keyword, alternate_keyword)]
#[belongs_to(Enumeration, foreign_key = "relationship_enum_id")]
#[table_name = "keyword_thesaurus"]
pub struct KeywordThesaurus{
    // keys
    pub entered_keyword: Option<String>,
    pub alternate_keyword: Option<String>,
    // fields
    pub relationship_enum_id: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(standard_language_id)]
#[table_name = "standard_language"]
pub struct StandardLanguage{
    // keys
    pub standard_language_id: i32,
    // fields
    pub lang_code_3t: Option<String>,
    pub lang_code_3b: Option<String>,
    pub lang_code_2: Option<String>,
    pub lang_name: Option<String>,
    pub lang_family: Option<String>,
    pub lang_charset: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_method_id)]
#[belongs_to(CustomMethodType, foreign_key = "custom_method_type_id")]
#[table_name = "custom_method"]
pub struct CustomMethod{
    // keys
    pub custom_method_id: i32,
    // fields
    pub custom_method_type_id: i32,
    pub custom_method_name: Option<String>,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_method_type_id)]
#[belongs_to(CustomMethodType, foreign_key = "parent_type_id")]
#[table_name = "custom_method_type"]
pub struct CustomMethodType{
    // keys
    pub custom_method_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(note_id)]
#[table_name = "note_data"]
pub struct NoteData{
    // keys
    pub note_id: i32,
    // fields
    pub note_name: Option<String>,
    pub note_info: Option<String>,
    pub note_date_time: chrono::NaiveDateTime
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_time_period_id)]
#[belongs_to(CustomTimePeriod, foreign_key = "parent_period_id")]
#[belongs_to(PeriodType, foreign_key = "period_type_id")]
#[table_name = "custom_time_period"]
pub struct CustomTimePeriod{
    // keys
    pub custom_time_period_id: i32,
    // fields
    pub parent_period_id: i32,
    pub period_type_id: i32,
    pub period_num: i64,
    pub period_name: Option<String>,
    pub from_date: chrono::NaiveDateTime,
    pub thru_date: chrono::NaiveDateTime,
    pub is_closed: bool
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(period_type_id)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[table_name = "period_type"]
pub struct PeriodType{
    // keys
    pub period_type_id: i32,
    // fields
    pub description: Option<String>,
    pub period_length: i64,
    pub uom_id: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_id)]
#[belongs_to(StatusType, foreign_key = "status_type_id")]
#[table_name = "status_item"]
pub struct StatusItem{
    // keys
    pub status_id: i32,
    // fields
    pub status_type_id: i32,
    pub status_code: Option<String>,
    pub sequence_id: i32,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_type_id)]
#[belongs_to(StatusType, foreign_key = "parent_type_id")]
#[table_name = "status_type"]
pub struct StatusType{
    // keys
    pub status_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_id, status_id_to)]
#[belongs_to(StatusItem, foreign_key = "status_id")]
#[belongs_to(StatusItem, foreign_key = "status_id_to")]
#[table_name = "status_valid_change"]
pub struct StatusValidChange{
    // keys
    pub status_id: i32,
    pub status_id_to: i32,
    // fields
    pub condition_expression: Option<String>,
    pub transition_name: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id)]
#[belongs_to(UomType, foreign_key = "uom_type_id")]
#[table_name = "uom"]
pub struct Uom{
    // keys
    pub uom_id: i32,
    // fields
    pub uom_type_id: i32,
    pub abbreviation: Option<String>,
    pub numeric_code: i64,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id, uom_id_to)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[belongs_to(Uom, foreign_key = "uom_id_to")]
#[belongs_to(CustomMethod, foreign_key = "custom_method_id")]
#[table_name = "uom_conversion"]
pub struct UomConversion{
    // keys
    pub uom_id: i32,
    pub uom_id_to: i32,
    // fields
    pub conversion_factor: f32,
    pub custom_method_id: i32,
    pub decimal_scale: i64,
    pub rounding_mode: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_id, uom_id_to, from_date)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[belongs_to(Uom, foreign_key = "uom_id_to")]
#[belongs_to(CustomMethod, foreign_key = "custom_method_id")]
#[belongs_to(Enumeration, foreign_key = "purpose_enum_id")]
#[table_name = "uom_conversion_dated"]
pub struct UomConversionDated{
    // keys
    pub uom_id: i32,
    pub uom_id_to: i32,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub conversion_factor: f32,
    pub custom_method_id: i32,
    pub decimal_scale: i64,
    pub rounding_mode: i32,
    pub purpose_enum_id: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_group_id, uom_id)]
#[belongs_to(Uom, foreign_key = "uom_id")]
#[table_name = "uom_group"]
pub struct UomGroup{
    // keys
    pub uom_group_id: i32,
    pub uom_id: i32,
    // fields
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(uom_type_id)]
#[belongs_to(UomType, foreign_key = "parent_type_id")]
#[table_name = "uom_type"]
pub struct UomType{
    // keys
    pub uom_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_login_id, user_pref_type_id)]
#[belongs_to(UserLogin, foreign_key = "user_login_id")]
#[belongs_to(UserPrefGroupType, foreign_key = "user_pref_group_type_id")]
#[table_name = "user_preference"]
pub struct UserPreference{
    // keys
    pub user_login_id: i64,
    pub user_pref_type_id: i64,
    // fields
    pub user_pref_group_type_id: i64,
    pub user_pref_value: Option<String>,
    pub user_pref_data_type: i64
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(user_pref_group_type_id)]
#[table_name = "user_pref_group_type"]
pub struct UserPrefGroupType{
    // keys
    pub user_pref_group_type_id: i64,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_screen_id)]
#[belongs_to(CustomScreenType, foreign_key = "custom_screen_type_id")]
#[table_name = "custom_screen"]
pub struct CustomScreen{
    // keys
    pub custom_screen_id: i32,
    // fields
    pub custom_screen_type_id: i32,
    pub custom_screen_name: Option<String>,
    pub custom_screen_location: Option<String>,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(custom_screen_type_id)]
#[table_name = "custom_screen_type"]
pub struct CustomScreenType{
    // keys
    pub custom_screen_type_id: i32,
    // fields
    pub parent_type_id: i32,
    pub has_table: bool,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(visual_theme_set_id)]
#[table_name = "visual_theme_set"]
pub struct VisualThemeSet{
    // keys
    pub visual_theme_set_id: i32,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(visual_theme_id)]
#[belongs_to(VisualThemeSet, foreign_key = "visual_theme_set_id")]
#[table_name = "visual_theme"]
pub struct VisualTheme{
    // keys
    pub visual_theme_id: i32,
    // fields
    pub visual_theme_set_id: i32,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(visual_theme_id, resource_type_enum_id, sequence_id)]
#[belongs_to(VisualTheme, foreign_key = "visual_theme_id")]
#[belongs_to(Enumeration, foreign_key = "resource_type_enum_id")]
#[table_name = "visual_theme_resource"]
pub struct VisualThemeResource{
    // keys
    pub visual_theme_id: i32,
    pub resource_type_enum_id: i32,
    pub sequence_id: i32,
    // fields
    pub resource_value: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_portlet_id)]
#[table_name = "portal_portlet"]
pub struct PortalPortlet{
    // keys
    pub portal_portlet_id: i32,
    // fields
    pub portlet_name: Option<String>,
    pub screen_name: Option<String>,
    pub screen_location: Option<String>,
    pub edit_form_name: Option<String>,
    pub edit_form_location: Option<String>,
    pub description: Option<String>,
    pub screenshot: Option<String>,
    pub security_service_name: Option<String>,
    pub security_main_action: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portlet_category_id)]
#[table_name = "portlet_category"]
pub struct PortletCategory{
    // keys
    pub portlet_category_id: i32,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_portlet_id, portlet_category_id)]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[belongs_to(PortletCategory, foreign_key = "portlet_category_id")]
#[table_name = "portlet_portlet_category"]
pub struct PortletPortletCategory{
    // keys
    pub portal_portlet_id: i32,
    pub portlet_category_id: i32,
    // fields
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id)]
#[belongs_to(PortalPage, foreign_key = "parent_portal_page_id")]
#[belongs_to(SecurityGroup, foreign_key = "security_group_id")]
#[table_name = "portal_page"]
pub struct PortalPage{
    // keys
    pub portal_page_id: i32,
    // fields
    pub portal_page_name: Option<String>,
    pub description: Option<String>,
    pub owner_user_login_id: i64,
    pub original_portal_page_id: i32,
    pub parent_portal_page_id: i32,
    pub sequence_num: i64,
    pub security_group_id: i32
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, column_seq_id)]
#[belongs_to(PortalPage, foreign_key = "portal_page_id")]
#[table_name = "portal_page_column"]
pub struct PortalPageColumn{
    // keys
    pub portal_page_id: i32,
    pub column_seq_id: i32,
    // fields
    pub column_width_pixels: i64,
    pub column_width_percentage: i64
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, portal_portlet_id, portlet_seq_id)]
#[belongs_to(PortalPage, foreign_key = "portal_page_id")]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[table_name = "portal_page_portlet"]
pub struct PortalPagePortlet{
    // keys
    pub portal_page_id: i32,
    pub portal_portlet_id: i32,
    pub portlet_seq_id: i32,
    // fields
    pub column_seq_id: i32,
    pub sequence_num: i64
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(portal_page_id, portal_portlet_id, portlet_seq_id, attr_name)]
#[belongs_to(PortalPortlet, foreign_key = "portal_portlet_id")]
#[table_name = "portlet_attribute"]
pub struct PortletAttribute{
    // keys
    pub portal_page_id: i32,
    pub portal_portlet_id: i32,
    pub portlet_seq_id: i32,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>,
    pub attr_type: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(system_resource_id, system_property_id)]
#[table_name = "system_property"]
pub struct SystemProperty{
    // keys
    pub system_resource_id: i64,
    pub system_property_id: i64,
    // fields
    pub system_property_value: Option<String>,
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(telecom_method_type_id)]
#[table_name = "telecom_method_type"]
pub struct TelecomMethodType{
    // keys
    pub telecom_method_type_id: i32,
    // fields
    pub description: Option<String>
}
        
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(telecom_gateway_config_id)]
#[table_name = "telecom_gateway_config"]
pub struct TelecomGatewayConfig{
    // keys
    pub telecom_gateway_config_id: i32,
    // fields
    pub description: Option<String>
}
        