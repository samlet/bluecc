use serde_derive::{Deserialize, Serialize};
// use crate::schema::party;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "addendum"]
pub struct Addendum{
    // keys
    pub addendum_id: Option<i64>, // id
    // fields
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub addendum_creation_date: Option<chrono::NaiveDateTime>, // date-time
    pub addendum_effective_date: Option<chrono::NaiveDateTime>, // date-time
    pub addendum_text: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement"]
pub struct Agreement{
    // keys
    pub agreement_id: Option<i64>, // id
    // fields
    pub product_id: Option<i64>, // id
    pub party_id_from: Option<i64>, // id
    pub party_id_to: Option<i64>, // id
    pub role_type_id_from: Option<i64>, // id
    pub role_type_id_to: Option<i64>, // id
    pub agreement_type_id: Option<i64>, // id
    pub agreement_date: Option<chrono::NaiveDateTime>, // date-time
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub description: Option<String>, // description
    pub text_data: Option<String> // very-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_attribute"]
pub struct AgreementAttribute{
    // keys
    pub agreement_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_geographical_applic"]
pub struct AgreementGeographicalApplic{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub geo_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_item"]
pub struct AgreementItem{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    // fields
    pub agreement_item_type_id: Option<i64>, // id
    pub currency_uom_id: Option<i64>, // id
    pub agreement_text: Option<String>, // very-long
    pub agreement_image: Option<Vec<u8>> // object
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_item_attribute"]
pub struct AgreementItemAttribute{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_item_type"]
pub struct AgreementItemType{
    // keys
    pub agreement_item_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_item_type_attr"]
pub struct AgreementItemTypeAttr{
    // keys
    pub agreement_item_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_content"]
pub struct AgreementContent{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub agreement_content_type_id: Option<i64>, // id
    pub content_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_content_type"]
pub struct AgreementContentType{
    // keys
    pub agreement_content_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_party_applic"]
pub struct AgreementPartyApplic{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub party_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_product_appl"]
pub struct AgreementProductAppl{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub product_id: Option<i64>, // id
    // fields
    pub price: Option<bigdecimal::BigDecimal> // currency-precise
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_promo_appl"]
pub struct AgreementPromoAppl{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub product_promo_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub sequence_num: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_facility_appl"]
pub struct AgreementFacilityAppl{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub facility_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_role"]
pub struct AgreementRole{
    // keys
    pub agreement_id: Option<i64>, // id
    pub party_id: Option<i64>, // id
    pub role_type_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_term"]
pub struct AgreementTerm{
    // keys
    pub agreement_term_id: Option<i64>, // id
    // fields
    pub term_type_id: Option<i64>, // id
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub invoice_item_type_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub term_value: Option<bigdecimal::BigDecimal>, // currency-precise
    pub term_days: Option<i64>, // numeric
    pub text_value: Option<String>, // description
    pub min_quantity: Option<bigdecimal::BigDecimal>, // floating-point
    pub max_quantity: Option<bigdecimal::BigDecimal>, // floating-point
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_term_attribute"]
pub struct AgreementTermAttribute{
    // keys
    pub agreement_term_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_type"]
pub struct AgreementType{
    // keys
    pub agreement_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_type_attr"]
pub struct AgreementTypeAttr{
    // keys
    pub agreement_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_work_effort_applic"]
pub struct AgreementWorkEffortApplic{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub work_effort_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "term_type"]
pub struct TermType{
    // keys
    pub term_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "term_type_attr"]
pub struct TermTypeAttr{
    // keys
    pub term_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "agreement_employment_appl"]
pub struct AgreementEmploymentAppl{
    // keys
    pub agreement_id: Option<i64>, // id
    pub agreement_item_seq_id: Option<i64>, // id
    pub party_id_from: Option<i64>, // id
    pub party_id_to: Option<i64>, // id
    pub role_type_id_from: Option<i64>, // id
    pub role_type_id_to: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub agreement_date: Option<chrono::NaiveDateTime>, // date-time
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "comm_content_assoc_type"]
pub struct CommContentAssocType{
    // keys
    pub comm_content_assoc_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "comm_event_content_assoc"]
pub struct CommEventContentAssoc{
    // keys
    pub content_id: Option<i64>, // id
    pub communication_event_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub comm_content_assoc_type_id: Option<i64>, // id
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub sequence_num: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event"]
pub struct CommunicationEvent{
    // keys
    pub communication_event_id: Option<i64>, // id
    // fields
    pub communication_event_type_id: Option<i64>, // id
    pub orig_comm_event_id: Option<i64>, // id
    pub parent_comm_event_id: Option<i64>, // id
    pub status_id: Option<i64>, // id
    pub contact_mech_type_id: Option<i64>, // id
    pub contact_mech_id_from: Option<i64>, // id
    pub contact_mech_id_to: Option<i64>, // id
    pub role_type_id_from: Option<i64>, // id
    pub role_type_id_to: Option<i64>, // id
    pub party_id_from: Option<i64>, // id
    pub party_id_to: Option<i64>, // id
    pub entry_date: Option<chrono::NaiveDateTime>, // date-time
    pub datetime_started: Option<chrono::NaiveDateTime>, // date-time
    pub datetime_ended: Option<chrono::NaiveDateTime>, // date-time
    pub subject: Option<String>, // long-varchar
    pub content_mime_type_id: Option<i64>, // id-vlong
    pub content: Option<String>, // very-long
    pub note: Option<String>, // comment
    pub reason_enum_id: Option<i64>, // id
    pub contact_list_id: Option<i64>, // id
    pub header_string: Option<String>, // very-long
    pub from_string: Option<String>, // very-long
    pub to_string: Option<String>, // very-long
    pub cc_string: Option<String>, // very-long
    pub bcc_string: Option<String>, // very-long
    pub message_id: Option<String> // value
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event_product"]
pub struct CommunicationEventProduct{
    // keys
    pub product_id: Option<i64>, // id
    pub communication_event_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event_prp_typ"]
pub struct CommunicationEventPrpTyp{
    // keys
    pub communication_event_prp_typ_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event_purpose"]
pub struct CommunicationEventPurpose{
    // keys
    pub communication_event_prp_typ_id: Option<i64>, // id
    pub communication_event_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event_role"]
pub struct CommunicationEventRole{
    // keys
    pub communication_event_id: Option<i64>, // id
    pub party_id: Option<i64>, // id
    pub role_type_id: Option<i64>, // id
    // fields
    pub contact_mech_id: Option<i64>, // id
    pub status_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "communication_event_type"]
pub struct CommunicationEventType{
    // keys
    pub communication_event_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String>, // description
    pub contact_mech_type_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech"]
pub struct ContactMech{
    // keys
    pub contact_mech_id: Option<i64>, // id
    // fields
    pub contact_mech_type_id: Option<i64>, // id
    pub info_string: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_attribute"]
pub struct ContactMechAttribute{
    // keys
    pub contact_mech_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_link"]
pub struct ContactMechLink{
    // keys
    pub contact_mech_id_from: Option<i64>, // id
    pub contact_mech_id_to: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_purpose_type"]
pub struct ContactMechPurposeType{
    // keys
    pub contact_mech_purpose_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_type"]
pub struct ContactMechType{
    // keys
    pub contact_mech_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_type_attr"]
pub struct ContactMechTypeAttr{
    // keys
    pub contact_mech_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "contact_mech_type_purpose"]
pub struct ContactMechTypePurpose{
    // keys
    pub contact_mech_type_id: Option<i64>, // id
    pub contact_mech_purpose_type_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "email_address_verification"]
pub struct EmailAddressVerification{
    // keys
    pub email_address: Option<i64>, // id-vlong
    // fields
    pub verify_hash: Option<String>, // value
    pub expire_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_contact_mech"]
pub struct PartyContactMech{
    // keys
    pub party_id: Option<i64>, // id
    pub contact_mech_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub role_type_id: Option<i64>, // id
    pub allow_solicitation: Option<bool>, // indicator
    pub extension: Option<String>, // long-varchar
    pub verified: Option<bool>, // indicator
    pub comments: Option<String>, // comment
    pub years_with_contact_mech: Option<i64>, // numeric
    pub months_with_contact_mech: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_contact_mech_purpose"]
pub struct PartyContactMechPurpose{
    // keys
    pub party_id: Option<i64>, // id
    pub contact_mech_id: Option<i64>, // id
    pub contact_mech_purpose_type_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "postal_address"]
pub struct PostalAddress{
    // keys
    pub contact_mech_id: Option<i64>, // id
    // fields
    pub to_name: Option<String>, // name
    pub attn_name: Option<String>, // name
    pub address_1: Option<String>, // long-varchar
    pub address_2: Option<String>, // long-varchar
    pub house_number: Option<i64>, // numeric
    pub house_number_ext: Option<String>, // short-varchar
    pub directions: Option<String>, // long-varchar
    pub city: Option<String>, // name
    pub city_geo_id: Option<i64>, // id
    pub postal_code: Option<String>, // short-varchar
    pub postal_code_ext: Option<String>, // short-varchar
    pub country_geo_id: Option<i64>, // id
    pub state_province_geo_id: Option<i64>, // id
    pub county_geo_id: Option<i64>, // id
    pub municipality_geo_id: Option<i64>, // id
    pub postal_code_geo_id: Option<i64>, // id
    pub geo_point_id: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "postal_address_boundary"]
pub struct PostalAddressBoundary{
    // keys
    pub contact_mech_id: Option<i64>, // id
    pub geo_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "telecom_number"]
pub struct TelecomNumber{
    // keys
    pub contact_mech_id: Option<i64>, // id
    // fields
    pub country_code: Option<String>, // very-short
    pub area_code: Option<String>, // very-short
    pub contact_number: Option<String>, // short-varchar
    pub ask_for_name: Option<String> // name
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "ftp_address"]
pub struct FtpAddress{
    // keys
    pub contact_mech_id: Option<i64>, // id
    // fields
    pub hostname: Option<String>, // long-varchar
    pub port: Option<i64>, // numeric
    pub username: Option<String>, // long-varchar
    pub ftp_password: Option<String>, // long-varchar
    pub binary_transfer: Option<bool>, // indicator
    pub file_path: Option<String>, // long-varchar
    pub zip_file: Option<bool>, // indicator
    pub passive_mode: Option<bool>, // indicator
    pub default_timeout: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "valid_contact_mech_role"]
pub struct ValidContactMechRole{
    // keys
    pub role_type_id: Option<i64>, // id
    pub contact_mech_type_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "need_type"]
pub struct NeedType{
    // keys
    pub need_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_need"]
pub struct PartyNeed{
    // keys
    pub party_need_id: Option<i64>, // id
    pub party_id: Option<i64>, // id
    pub role_type_id: Option<i64>, // id
    // fields
    pub party_type_id: Option<i64>, // id
    pub need_type_id: Option<i64>, // id
    pub communication_event_id: Option<i64>, // id
    pub product_id: Option<i64>, // id
    pub product_category_id: Option<i64>, // id
    pub visit_id: Option<i64>, // id
    pub datetime_recorded: Option<chrono::NaiveDateTime>, // date-time
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "address_match_map"]
pub struct AddressMatchMap{
    // keys
    pub map_key: Option<i64>, // id-vlong
    pub map_value: Option<i64>, // id-vlong
    // fields
    pub sequence_num: Option<i64> // numeric
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "affiliate"]
pub struct Affiliate{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub affiliate_name: Option<String>, // name
    pub affiliate_description: Option<String>, // description
    pub year_established: Option<String>, // very-short
    pub site_type: Option<String>, // comment
    pub site_page_views: Option<String>, // comment
    pub site_visitors: Option<String>, // comment
    pub date_time_created: Option<chrono::NaiveDateTime>, // date-time
    pub date_time_approved: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party"]
pub struct Party{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub party_type_id: Option<i64>, // id
    pub external_id: Option<i64>, // id
    pub preferred_currency_uom_id: Option<i64>, // id
    pub description: Option<String>, // very-long
    pub status_id: Option<i64>, // id
    pub created_date: Option<chrono::NaiveDateTime>, // date-time
    pub created_by_user_login: Option<i64>, // id-vlong
    pub last_modified_date: Option<chrono::NaiveDateTime>, // date-time
    pub last_modified_by_user_login: Option<i64>, // id-vlong
    pub data_source_id: Option<i64>, // id
    pub is_unread: Option<bool> // indicator
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_identification"]
pub struct PartyIdentification{
    // keys
    pub party_id: Option<i64>, // id
    pub party_identification_type_id: Option<i64>, // id
    // fields
    pub id_value: Option<i64> // id-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_identification_type"]
pub struct PartyIdentificationType{
    // keys
    pub party_identification_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_geo_point"]
pub struct PartyGeoPoint{
    // keys
    pub party_id: Option<i64>, // id
    pub geo_point_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_attribute"]
pub struct PartyAttribute{
    // keys
    pub party_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub attr_value: Option<String>, // value
    pub attr_description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_carrier_account"]
pub struct PartyCarrierAccount{
    // keys
    pub party_id: Option<i64>, // id
    pub carrier_party_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub account_number: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_classification"]
pub struct PartyClassification{
    // keys
    pub party_id: Option<i64>, // id
    pub party_classification_group_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_classification_group"]
pub struct PartyClassificationGroup{
    // keys
    pub party_classification_group_id: Option<i64>, // id
    // fields
    pub party_classification_type_id: Option<i64>, // id
    pub parent_group_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_classification_type"]
pub struct PartyClassificationType{
    // keys
    pub party_classification_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_content"]
pub struct PartyContent{
    // keys
    pub party_id: Option<i64>, // id
    pub content_id: Option<i64>, // id
    pub party_content_type_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_content_type"]
pub struct PartyContentType{
    // keys
    pub party_content_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_data_source"]
pub struct PartyDataSource{
    // keys
    pub party_id: Option<i64>, // id
    pub data_source_id: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub visit_id: Option<i64>, // id
    pub comments: Option<String>, // comment
    pub is_create: Option<bool> // indicator
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_group"]
pub struct PartyGroup{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub group_name: Option<String>, // name
    pub group_name_local: Option<String>, // name
    pub office_site_name: Option<String>, // name
    pub annual_revenue: Option<bigdecimal::BigDecimal>, // currency-amount
    pub num_employees: Option<i64>, // numeric
    pub ticker_symbol: Option<String>, // very-short
    pub comments: Option<String>, // comment
    pub logo_image_url: Option<String> // url
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_ics_avs_override"]
pub struct PartyIcsAvsOverride{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub avs_decline_string: Option<String> // long-varchar
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_invitation"]
pub struct PartyInvitation{
    // keys
    pub party_invitation_id: Option<i64>, // id
    // fields
    pub party_id_from: Option<i64>, // id
    pub party_id: Option<i64>, // id
    pub to_name: Option<String>, // name
    pub email_address: Option<String>, // long-varchar
    pub status_id: Option<i64>, // id
    pub last_invite_date: Option<chrono::NaiveDateTime> // date-time
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_invitation_group_assoc"]
pub struct PartyInvitationGroupAssoc{
    // keys
    pub party_invitation_id: Option<i64>, // id
    pub party_id_to: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_invitation_role_assoc"]
pub struct PartyInvitationRoleAssoc{
    // keys
    pub party_invitation_id: Option<i64>, // id
    pub role_type_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_name_history"]
pub struct PartyNameHistory{
    // keys
    pub party_id: Option<i64>, // id
    pub change_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub group_name: Option<String>, // name
    pub first_name: Option<String>, // name
    pub middle_name: Option<String>, // name
    pub last_name: Option<String>, // name
    pub personal_title: Option<String>, // name
    pub suffix: Option<String> // name
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_note"]
pub struct PartyNote{
    // keys
    pub party_id: Option<i64>, // id
    pub note_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_profile_default"]
pub struct PartyProfileDefault{
    // keys
    pub party_id: Option<i64>, // id
    pub product_store_id: Option<i64>, // id
    // fields
    pub default_ship_addr: Option<i64>, // id
    pub default_bill_addr: Option<i64>, // id
    pub default_pay_meth: Option<i64>, // id
    pub default_ship_meth: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_relationship"]
pub struct PartyRelationship{
    // keys
    pub party_id_from: Option<i64>, // id
    pub party_id_to: Option<i64>, // id
    pub role_type_id_from: Option<i64>, // id
    pub role_type_id_to: Option<i64>, // id
    pub from_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub thru_date: Option<chrono::NaiveDateTime>, // date-time
    pub status_id: Option<i64>, // id
    pub relationship_name: Option<String>, // name
    pub security_group_id: Option<i64>, // id
    pub priority_type_id: Option<i64>, // id
    pub party_relationship_type_id: Option<i64>, // id
    pub permissions_enum_id: Option<i64>, // id
    pub position_title: Option<String>, // name
    pub comments: Option<String> // comment
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_relationship_type"]
pub struct PartyRelationshipType{
    // keys
    pub party_relationship_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub party_relationship_name: Option<String>, // name
    pub description: Option<String>, // description
    pub role_type_id_valid_from: Option<i64>, // id
    pub role_type_id_valid_to: Option<i64> // id
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_role"]
pub struct PartyRole{
    // keys
    pub party_id: Option<i64>, // id
    pub role_type_id: Option<i64>, // id
    // fields
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_status"]
pub struct PartyStatus{
    // keys
    pub status_id: Option<i64>, // id
    pub party_id: Option<i64>, // id
    pub status_date: Option<chrono::NaiveDateTime>, // date-time
    // fields
    pub change_by_user_login_id: Option<i64> // id-vlong
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_type"]
pub struct PartyType{
    // keys
    pub party_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "party_type_attr"]
pub struct PartyTypeAttr{
    // keys
    pub party_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "person"]
pub struct Person{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub salutation: Option<String>, // name
    pub first_name: Option<String>, // name
    pub middle_name: Option<String>, // name
    pub last_name: Option<String>, // name
    pub personal_title: Option<String>, // name
    pub suffix: Option<String>, // name
    pub nickname: Option<String>, // name
    pub first_name_local: Option<String>, // name
    pub middle_name_local: Option<String>, // name
    pub last_name_local: Option<String>, // name
    pub other_local: Option<String>, // name
    pub member_id: Option<i64>, // id
    pub gender: Option<bool>, // indicator
    pub birth_date: Option<chrono::NaiveDate>, // date
    pub deceased_date: Option<chrono::NaiveDate>, // date
    pub height: Option<bigdecimal::BigDecimal>, // floating-point
    pub weight: Option<bigdecimal::BigDecimal>, // floating-point
    pub mothers_maiden_name: Option<String>, // long-varchar
    pub old_marital_status: Option<bool>, // indicator
    pub marital_status_enum_id: Option<i64>, // id
    pub social_security_number: Option<String>, // long-varchar
    pub passport_number: Option<String>, // long-varchar
    pub passport_expire_date: Option<chrono::NaiveDate>, // date
    pub total_years_work_experience: Option<bigdecimal::BigDecimal>, // floating-point
    pub comments: Option<String>, // comment
    pub employment_status_enum_id: Option<i64>, // id
    pub residence_status_enum_id: Option<i64>, // id
    pub occupation: Option<String>, // name
    pub years_with_employer: Option<i64>, // numeric
    pub months_with_employer: Option<i64>, // numeric
    pub existing_customer: Option<bool>, // indicator
    pub card_id: Option<i64> // id-long
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "priority_type"]
pub struct PriorityType{
    // keys
    pub priority_type_id: Option<i64>, // id
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "role_type"]
pub struct RoleType{
    // keys
    pub role_type_id: Option<i64>, // id
    // fields
    pub parent_type_id: Option<i64>, // id
    pub has_table: Option<bool>, // indicator
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "role_type_attr"]
pub struct RoleTypeAttr{
    // keys
    pub role_type_id: Option<i64>, // id
    pub attr_name: Option<i64>, // id-long
    // fields
    pub description: Option<String> // description
}

#[derive(Debug, Insertable, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[table_name = "vendor"]
pub struct Vendor{
    // keys
    pub party_id: Option<i64>, // id
    // fields
    pub manifest_company_name: Option<String>, // name
    pub manifest_company_title: Option<String>, // name
    pub manifest_logo_url: Option<String>, // url
    pub manifest_policies: Option<String> // very-long
}

