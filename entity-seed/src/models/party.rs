use serde_derive::{Deserialize, Serialize};
// use crate::schema::party;
use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(addendum_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "addendum"]
pub struct Addendum{
    // keys
    pub addendum_id: i64,
    // fields
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub addendum_creation_date: chrono::NaiveDateTime,
    pub addendum_effective_date: chrono::NaiveDateTime,
    pub addendum_text: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id)]
#[belongs_to(Party, foreign_key = "party_id_from")]
#[belongs_to(RoleType, foreign_key = "role_type_id_from")]
#[belongs_to(AgreementType, foreign_key = "agreement_type_id")]
#[table_name = "agreement"]
pub struct Agreement{
    // keys
    pub agreement_id: i64,
    // fields
    pub product_id: i64,
    pub party_id_from: i64,
    pub party_id_to: i64,
    pub role_type_id_from: i64,
    pub role_type_id_to: i64,
    pub agreement_type_id: i64,
    pub agreement_date: chrono::NaiveDateTime,
    pub from_date: chrono::NaiveDateTime,
    pub thru_date: chrono::NaiveDateTime,
    pub description: Option<String>,
    pub text_data: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, attr_name)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_attribute"]
pub struct AgreementAttribute{
    // keys
    pub agreement_id: i64,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, geo_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_geographical_applic"]
pub struct AgreementGeographicalApplic{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub geo_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[belongs_to(AgreementItemType, foreign_key = "agreement_item_type_id")]
#[table_name = "agreement_item"]
pub struct AgreementItem{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    // fields
    pub agreement_item_type_id: i64,
    pub currency_uom_id: i64,
    pub agreement_text: Option<String>,
    pub agreement_image: Vec<u8>
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(agreement_id, agreement_item_seq_id, attr_name)]
#[table_name = "agreement_item_attribute"]
pub struct AgreementItemAttribute{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_item_type_id)]
#[belongs_to(AgreementItemType, foreign_key = "parent_type_id")]
#[table_name = "agreement_item_type"]
pub struct AgreementItemType{
    // keys
    pub agreement_item_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_item_type_id, attr_name)]
#[belongs_to(AgreementItemType, foreign_key = "agreement_item_type_id")]
#[table_name = "agreement_item_type_attr"]
pub struct AgreementItemTypeAttr{
    // keys
    pub agreement_item_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(content_id, agreement_id, agreement_item_seq_id, agreement_content_type_id, from_date)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[belongs_to(AgreementContentType, foreign_key = "agreement_content_type_id")]
#[table_name = "agreement_content"]
pub struct AgreementContent{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub agreement_content_type_id: i64,
    pub content_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_content_type_id)]
#[belongs_to(AgreementContentType, foreign_key = "parent_type_id")]
#[table_name = "agreement_content_type"]
pub struct AgreementContentType{
    // keys
    pub agreement_content_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, party_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "agreement_party_applic"]
pub struct AgreementPartyApplic{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub party_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, product_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_product_appl"]
pub struct AgreementProductAppl{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub product_id: i64,
    // fields
    pub price: bigdecimal::BigDecimal
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, product_promo_id, from_date)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_promo_appl"]
pub struct AgreementPromoAppl{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub product_promo_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub sequence_num: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, facility_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_facility_appl"]
pub struct AgreementFacilityAppl{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub facility_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, party_id, role_type_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[table_name = "agreement_role"]
pub struct AgreementRole{
    // keys
    pub agreement_id: i64,
    pub party_id: i64,
    pub role_type_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_term_id)]
#[belongs_to(TermType, foreign_key = "term_type_id")]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_term"]
pub struct AgreementTerm{
    // keys
    pub agreement_term_id: i64,
    // fields
    pub term_type_id: i64,
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub invoice_item_type_id: i64,
    pub from_date: chrono::NaiveDateTime,
    pub thru_date: chrono::NaiveDateTime,
    pub term_value: bigdecimal::BigDecimal,
    pub term_days: i64,
    pub text_value: Option<String>,
    pub min_quantity: bigdecimal::BigDecimal,
    pub max_quantity: bigdecimal::BigDecimal,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_term_id, attr_name)]
#[belongs_to(AgreementTerm, foreign_key = "agreement_term_id")]
#[table_name = "agreement_term_attribute"]
pub struct AgreementTermAttribute{
    // keys
    pub agreement_term_id: i64,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_type_id)]
#[belongs_to(AgreementType, foreign_key = "parent_type_id")]
#[table_name = "agreement_type"]
pub struct AgreementType{
    // keys
    pub agreement_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_type_id, attr_name)]
#[belongs_to(AgreementType, foreign_key = "agreement_type_id")]
#[table_name = "agreement_type_attr"]
pub struct AgreementTypeAttr{
    // keys
    pub agreement_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(agreement_id, agreement_item_seq_id, work_effort_id)]
#[belongs_to(Agreement, foreign_key = "agreement_id")]
#[table_name = "agreement_work_effort_applic"]
pub struct AgreementWorkEffortApplic{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub work_effort_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(term_type_id)]
#[belongs_to(TermType, foreign_key = "parent_type_id")]
#[table_name = "term_type"]
pub struct TermType{
    // keys
    pub term_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(term_type_id, attr_name)]
#[belongs_to(TermType, foreign_key = "term_type_id")]
#[table_name = "term_type_attr"]
pub struct TermTypeAttr{
    // keys
    pub term_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(agreement_id, agreement_item_seq_id, party_id_to, party_id_from, role_type_id_to, role_type_id_from, from_date)]
#[table_name = "agreement_employment_appl"]
pub struct AgreementEmploymentAppl{
    // keys
    pub agreement_id: i64,
    pub agreement_item_seq_id: i64,
    pub party_id_from: i64,
    pub party_id_to: i64,
    pub role_type_id_from: i64,
    pub role_type_id_to: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub agreement_date: chrono::NaiveDateTime,
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(comm_content_assoc_type_id)]
#[table_name = "comm_content_assoc_type"]
pub struct CommContentAssocType{
    // keys
    pub comm_content_assoc_type_id: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(content_id, communication_event_id, from_date)]
#[belongs_to(CommunicationEvent, foreign_key = "communication_event_id")]
#[belongs_to(CommContentAssocType, foreign_key = "comm_content_assoc_type_id")]
#[table_name = "comm_event_content_assoc"]
pub struct CommEventContentAssoc{
    // keys
    pub content_id: i64,
    pub communication_event_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub comm_content_assoc_type_id: i64,
    pub thru_date: chrono::NaiveDateTime,
    pub sequence_num: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(communication_event_id)]
#[belongs_to(CommunicationEventType, foreign_key = "communication_event_type_id")]
#[belongs_to(Party, foreign_key = "party_id_to")]
#[belongs_to(RoleType, foreign_key = "role_type_id_to")]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id_from")]
#[table_name = "communication_event"]
pub struct CommunicationEvent{
    // keys
    pub communication_event_id: i64,
    // fields
    pub communication_event_type_id: i64,
    pub orig_comm_event_id: i64,
    pub parent_comm_event_id: i64,
    pub status_id: i64,
    pub contact_mech_type_id: i64,
    pub contact_mech_id_from: i64,
    pub contact_mech_id_to: i64,
    pub role_type_id_from: i64,
    pub role_type_id_to: i64,
    pub party_id_from: i64,
    pub party_id_to: i64,
    pub entry_date: chrono::NaiveDateTime,
    pub datetime_started: chrono::NaiveDateTime,
    pub datetime_ended: chrono::NaiveDateTime,
    pub subject: Option<String>,
    pub content_mime_type_id: i64,
    pub content: Option<String>,
    pub note: Option<String>,
    pub reason_enum_id: i64,
    pub contact_list_id: i64,
    pub header_string: Option<String>,
    pub from_string: Option<String>,
    pub to_string: Option<String>,
    pub cc_string: Option<String>,
    pub bcc_string: Option<String>,
    pub message_id: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(product_id, communication_event_id)]
#[belongs_to(CommunicationEvent, foreign_key = "communication_event_id")]
#[table_name = "communication_event_product"]
pub struct CommunicationEventProduct{
    // keys
    pub product_id: i64,
    pub communication_event_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(communication_event_prp_typ_id)]
#[belongs_to(CommunicationEventPrpTyp, foreign_key = "parent_type_id")]
#[table_name = "communication_event_prp_typ"]
pub struct CommunicationEventPrpTyp{
    // keys
    pub communication_event_prp_typ_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(communication_event_prp_typ_id, communication_event_id)]
#[belongs_to(CommunicationEvent, foreign_key = "communication_event_id")]
#[belongs_to(CommunicationEventPrpTyp, foreign_key = "communication_event_prp_typ_id")]
#[table_name = "communication_event_purpose"]
pub struct CommunicationEventPurpose{
    // keys
    pub communication_event_prp_typ_id: i64,
    pub communication_event_id: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(communication_event_id, party_id, role_type_id)]
#[belongs_to(CommunicationEvent, foreign_key = "communication_event_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[table_name = "communication_event_role"]
pub struct CommunicationEventRole{
    // keys
    pub communication_event_id: i64,
    pub party_id: i64,
    pub role_type_id: i64,
    // fields
    pub contact_mech_id: i64,
    pub status_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(communication_event_type_id)]
#[belongs_to(CommunicationEventType, foreign_key = "parent_type_id")]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[table_name = "communication_event_type"]
pub struct CommunicationEventType{
    // keys
    pub communication_event_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>,
    pub contact_mech_type_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id)]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[table_name = "contact_mech"]
pub struct ContactMech{
    // keys
    pub contact_mech_id: i64,
    // fields
    pub contact_mech_type_id: i64,
    pub info_string: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id, attr_name)]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[table_name = "contact_mech_attribute"]
pub struct ContactMechAttribute{
    // keys
    pub contact_mech_id: i64,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id_from, contact_mech_id_to)]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id_from")]
#[table_name = "contact_mech_link"]
pub struct ContactMechLink{
    // keys
    pub contact_mech_id_from: i64,
    pub contact_mech_id_to: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(contact_mech_purpose_type_id)]
#[table_name = "contact_mech_purpose_type"]
pub struct ContactMechPurposeType{
    // keys
    pub contact_mech_purpose_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_type_id)]
#[belongs_to(ContactMechType, foreign_key = "parent_type_id")]
#[table_name = "contact_mech_type"]
pub struct ContactMechType{
    // keys
    pub contact_mech_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_type_id, attr_name)]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[table_name = "contact_mech_type_attr"]
pub struct ContactMechTypeAttr{
    // keys
    pub contact_mech_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_type_id, contact_mech_purpose_type_id)]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[belongs_to(ContactMechPurposeType, foreign_key = "contact_mech_purpose_type_id")]
#[table_name = "contact_mech_type_purpose"]
pub struct ContactMechTypePurpose{
    // keys
    pub contact_mech_type_id: i64,
    pub contact_mech_purpose_type_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(email_address)]
#[table_name = "email_address_verification"]
pub struct EmailAddressVerification{
    // keys
    pub email_address: i64,
    // fields
    pub verify_hash: Option<String>,
    pub expire_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, contact_mech_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(Person, foreign_key = "party_id")]
#[belongs_to(PartyGroup, foreign_key = "party_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[belongs_to(TelecomNumber, foreign_key = "contact_mech_id")]
#[belongs_to(PostalAddress, foreign_key = "contact_mech_id")]
#[table_name = "party_contact_mech"]
pub struct PartyContactMech{
    // keys
    pub party_id: i64,
    pub contact_mech_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub role_type_id: i64,
    pub allow_solicitation: bool,
    pub extension: Option<String>,
    pub verified: bool,
    pub comments: Option<String>,
    pub years_with_contact_mech: i64,
    pub months_with_contact_mech: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, contact_mech_id, contact_mech_purpose_type_id, from_date)]
#[belongs_to(ContactMechPurposeType, foreign_key = "contact_mech_purpose_type_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(Person, foreign_key = "party_id")]
#[belongs_to(PartyGroup, foreign_key = "party_id")]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[belongs_to(PostalAddress, foreign_key = "contact_mech_id")]
#[belongs_to(TelecomNumber, foreign_key = "contact_mech_id")]
#[table_name = "party_contact_mech_purpose"]
pub struct PartyContactMechPurpose{
    // keys
    pub party_id: i64,
    pub contact_mech_id: i64,
    pub contact_mech_purpose_type_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id)]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[table_name = "postal_address"]
pub struct PostalAddress{
    // keys
    pub contact_mech_id: i64,
    // fields
    pub to_name: Option<String>,
    pub attn_name: Option<String>,
    pub address_1: Option<String>,
    pub address_2: Option<String>,
    pub house_number: i64,
    pub house_number_ext: Option<String>,
    pub directions: Option<String>,
    pub city: Option<String>,
    pub city_geo_id: i64,
    pub postal_code: Option<String>,
    pub postal_code_ext: Option<String>,
    pub country_geo_id: i64,
    pub state_province_geo_id: i64,
    pub county_geo_id: i64,
    pub municipality_geo_id: i64,
    pub postal_code_geo_id: i64,
    pub geo_point_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id, geo_id)]
#[belongs_to(PostalAddress, foreign_key = "contact_mech_id")]
#[table_name = "postal_address_boundary"]
pub struct PostalAddressBoundary{
    // keys
    pub contact_mech_id: i64,
    pub geo_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id)]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[table_name = "telecom_number"]
pub struct TelecomNumber{
    // keys
    pub contact_mech_id: i64,
    // fields
    pub country_code: Option<String>,
    pub area_code: Option<String>,
    pub contact_number: Option<String>,
    pub ask_for_name: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(contact_mech_id)]
#[belongs_to(ContactMech, foreign_key = "contact_mech_id")]
#[table_name = "ftp_address"]
pub struct FtpAddress{
    // keys
    pub contact_mech_id: i64,
    // fields
    pub hostname: Option<String>,
    pub port: i64,
    pub username: Option<String>,
    pub ftp_password: Option<String>,
    pub binary_transfer: bool,
    pub file_path: Option<String>,
    pub zip_file: bool,
    pub passive_mode: bool,
    pub default_timeout: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(role_type_id, contact_mech_type_id)]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[belongs_to(ContactMechType, foreign_key = "contact_mech_type_id")]
#[table_name = "valid_contact_mech_role"]
pub struct ValidContactMechRole{
    // keys
    pub role_type_id: i64,
    pub contact_mech_type_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(need_type_id)]
#[table_name = "need_type"]
pub struct NeedType{
    // keys
    pub need_type_id: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_need_id, party_id, role_type_id)]
#[belongs_to(NeedType, foreign_key = "need_type_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[belongs_to(PartyType, foreign_key = "party_type_id")]
#[belongs_to(CommunicationEvent, foreign_key = "communication_event_id")]
#[table_name = "party_need"]
pub struct PartyNeed{
    // keys
    pub party_need_id: i64,
    pub party_id: i64,
    pub role_type_id: i64,
    // fields
    pub party_type_id: i64,
    pub need_type_id: i64,
    pub communication_event_id: i64,
    pub product_id: i64,
    pub product_category_id: i64,
    pub visit_id: i64,
    pub datetime_recorded: chrono::NaiveDateTime,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(map_key, map_value)]
#[table_name = "address_match_map"]
pub struct AddressMatchMap{
    // keys
    pub map_key: i64,
    pub map_value: i64,
    // fields
    pub sequence_num: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(PartyGroup, foreign_key = "party_id")]
#[table_name = "affiliate"]
pub struct Affiliate{
    // keys
    pub party_id: i64,
    // fields
    pub affiliate_name: Option<String>,
    pub affiliate_description: Option<String>,
    pub year_established: Option<String>,
    pub site_type: Option<String>,
    pub site_page_views: Option<String>,
    pub site_visitors: Option<String>,
    pub date_time_created: chrono::NaiveDateTime,
    pub date_time_approved: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(PartyType, foreign_key = "party_type_id")]
#[table_name = "party"]
pub struct Party{
    // keys
    pub party_id: i64,
    // fields
    pub party_type_id: i64,
    pub external_id: i64,
    pub preferred_currency_uom_id: i64,
    pub description: Option<String>,
    pub status_id: i64,
    pub created_date: chrono::NaiveDateTime,
    pub created_by_user_login: i64,
    pub last_modified_date: chrono::NaiveDateTime,
    pub last_modified_by_user_login: i64,
    pub data_source_id: i64,
    pub is_unread: bool
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, party_identification_type_id)]
#[belongs_to(PartyIdentificationType, foreign_key = "party_identification_type_id")]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_identification"]
pub struct PartyIdentification{
    // keys
    pub party_id: i64,
    pub party_identification_type_id: i64,
    // fields
    pub id_value: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_identification_type_id)]
#[belongs_to(PartyIdentificationType, foreign_key = "parent_type_id")]
#[table_name = "party_identification_type"]
pub struct PartyIdentificationType{
    // keys
    pub party_identification_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, geo_point_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_geo_point"]
pub struct PartyGeoPoint{
    // keys
    pub party_id: i64,
    pub geo_point_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, attr_name)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_attribute"]
pub struct PartyAttribute{
    // keys
    pub party_id: i64,
    pub attr_name: i64,
    // fields
    pub attr_value: Option<String>,
    pub attr_description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, carrier_party_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_carrier_account"]
pub struct PartyCarrierAccount{
    // keys
    pub party_id: i64,
    pub carrier_party_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub account_number: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, party_classification_group_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(PartyClassificationGroup, foreign_key = "party_classification_group_id")]
#[table_name = "party_classification"]
pub struct PartyClassification{
    // keys
    pub party_id: i64,
    pub party_classification_group_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_classification_group_id)]
#[belongs_to(PartyClassificationGroup, foreign_key = "parent_group_id")]
#[belongs_to(PartyClassificationType, foreign_key = "party_classification_type_id")]
#[table_name = "party_classification_group"]
pub struct PartyClassificationGroup{
    // keys
    pub party_classification_group_id: i64,
    // fields
    pub party_classification_type_id: i64,
    pub parent_group_id: i64,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_classification_type_id)]
#[belongs_to(PartyClassificationType, foreign_key = "parent_type_id")]
#[table_name = "party_classification_type"]
pub struct PartyClassificationType{
    // keys
    pub party_classification_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, content_id, party_content_type_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(PartyContentType, foreign_key = "party_content_type_id")]
#[table_name = "party_content"]
pub struct PartyContent{
    // keys
    pub party_id: i64,
    pub content_id: i64,
    pub party_content_type_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_content_type_id)]
#[belongs_to(PartyContentType, foreign_key = "parent_type_id")]
#[table_name = "party_content_type"]
pub struct PartyContentType{
    // keys
    pub party_content_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, data_source_id, from_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_data_source"]
pub struct PartyDataSource{
    // keys
    pub party_id: i64,
    pub data_source_id: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub visit_id: i64,
    pub comments: Option<String>,
    pub is_create: bool
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_group"]
pub struct PartyGroup{
    // keys
    pub party_id: i64,
    // fields
    pub group_name: Option<String>,
    pub group_name_local: Option<String>,
    pub office_site_name: Option<String>,
    pub annual_revenue: bigdecimal::BigDecimal,
    pub num_employees: i64,
    pub ticker_symbol: Option<String>,
    pub comments: Option<String>,
    pub logo_image_url: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_ics_avs_override"]
pub struct PartyIcsAvsOverride{
    // keys
    pub party_id: i64,
    // fields
    pub avs_decline_string: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_invitation_id)]
#[belongs_to(Party, foreign_key = "party_id_from")]
#[table_name = "party_invitation"]
pub struct PartyInvitation{
    // keys
    pub party_invitation_id: i64,
    // fields
    pub party_id_from: i64,
    pub party_id: i64,
    pub to_name: Option<String>,
    pub email_address: Option<String>,
    pub status_id: i64,
    pub last_invite_date: chrono::NaiveDateTime
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_invitation_id, party_id_to)]
#[belongs_to(PartyGroup, foreign_key = "party_id_to")]
#[belongs_to(Party, foreign_key = "party_id_to")]
#[belongs_to(PartyInvitation, foreign_key = "party_invitation_id")]
#[table_name = "party_invitation_group_assoc"]
pub struct PartyInvitationGroupAssoc{
    // keys
    pub party_invitation_id: i64,
    pub party_id_to: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_invitation_id, role_type_id)]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[belongs_to(PartyInvitation, foreign_key = "party_invitation_id")]
#[table_name = "party_invitation_role_assoc"]
pub struct PartyInvitationRoleAssoc{
    // keys
    pub party_invitation_id: i64,
    pub role_type_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, change_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_name_history"]
pub struct PartyNameHistory{
    // keys
    pub party_id: i64,
    pub change_date: chrono::NaiveDateTime,
    // fields
    pub group_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_title: Option<String>,
    pub suffix: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, note_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_note"]
pub struct PartyNote{
    // keys
    pub party_id: i64,
    pub note_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, product_store_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_profile_default"]
pub struct PartyProfileDefault{
    // keys
    pub party_id: i64,
    pub product_store_id: i64,
    // fields
    pub default_ship_addr: i64,
    pub default_bill_addr: i64,
    pub default_pay_meth: i64,
    pub default_ship_meth: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id_from, party_id_to, role_type_id_from, role_type_id_to, from_date)]
#[belongs_to(Party, foreign_key = "party_id_from")]
#[belongs_to(RoleType, foreign_key = "role_type_id_from")]
#[belongs_to(PriorityType, foreign_key = "priority_type_id")]
#[belongs_to(PartyRelationshipType, foreign_key = "party_relationship_type_id")]
#[table_name = "party_relationship"]
pub struct PartyRelationship{
    // keys
    pub party_id_from: i64,
    pub party_id_to: i64,
    pub role_type_id_from: i64,
    pub role_type_id_to: i64,
    pub from_date: chrono::NaiveDateTime,
    // fields
    pub thru_date: chrono::NaiveDateTime,
    pub status_id: i64,
    pub relationship_name: Option<String>,
    pub security_group_id: i64,
    pub priority_type_id: i64,
    pub party_relationship_type_id: i64,
    pub permissions_enum_id: i64,
    pub position_title: Option<String>,
    pub comments: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_relationship_type_id)]
#[belongs_to(PartyRelationshipType, foreign_key = "parent_type_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id_valid_from")]
#[table_name = "party_relationship_type"]
pub struct PartyRelationshipType{
    // keys
    pub party_relationship_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub party_relationship_name: Option<String>,
    pub description: Option<String>,
    pub role_type_id_valid_from: i64,
    pub role_type_id_valid_to: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id, role_type_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[table_name = "party_role"]
pub struct PartyRole{
    // keys
    pub party_id: i64,
    pub role_type_id: i64,
    // fields
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(status_id, party_id, status_date)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "party_status"]
pub struct PartyStatus{
    // keys
    pub status_id: i64,
    pub party_id: i64,
    pub status_date: chrono::NaiveDateTime,
    // fields
    pub change_by_user_login_id: i64
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_type_id)]
#[belongs_to(PartyType, foreign_key = "parent_type_id")]
#[table_name = "party_type"]
pub struct PartyType{
    // keys
    pub party_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_type_id, attr_name)]
#[belongs_to(PartyType, foreign_key = "party_type_id")]
#[table_name = "party_type_attr"]
pub struct PartyTypeAttr{
    // keys
    pub party_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "person"]
pub struct Person{
    // keys
    pub party_id: i64,
    // fields
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_title: Option<String>,
    pub suffix: Option<String>,
    pub nickname: Option<String>,
    pub first_name_local: Option<String>,
    pub middle_name_local: Option<String>,
    pub last_name_local: Option<String>,
    pub other_local: Option<String>,
    pub member_id: i64,
    pub gender: bool,
    pub birth_date: chrono::NaiveDate,
    pub deceased_date: chrono::NaiveDate,
    pub height: bigdecimal::BigDecimal,
    pub weight: bigdecimal::BigDecimal,
    pub mothers_maiden_name: Option<String>,
    pub old_marital_status: bool,
    pub marital_status_enum_id: i64,
    pub social_security_number: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expire_date: chrono::NaiveDate,
    pub total_years_work_experience: bigdecimal::BigDecimal,
    pub comments: Option<String>,
    pub employment_status_enum_id: i64,
    pub residence_status_enum_id: i64,
    pub occupation: Option<String>,
    pub years_with_employer: i64,
    pub months_with_employer: i64,
    pub existing_customer: bool,
    pub card_id: i64
}

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(priority_type_id)]
#[table_name = "priority_type"]
pub struct PriorityType{
    // keys
    pub priority_type_id: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(role_type_id)]
#[belongs_to(RoleType, foreign_key = "parent_type_id")]
#[table_name = "role_type"]
pub struct RoleType{
    // keys
    pub role_type_id: i64,
    // fields
    pub parent_type_id: i64,
    pub has_table: bool,
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(role_type_id, attr_name)]
#[belongs_to(RoleType, foreign_key = "role_type_id")]
#[table_name = "role_type_attr"]
pub struct RoleTypeAttr{
    // keys
    pub role_type_id: i64,
    pub attr_name: i64,
    // fields
    pub description: Option<String>
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[primary_key(party_id)]
#[belongs_to(Party, foreign_key = "party_id")]
#[table_name = "vendor"]
pub struct Vendor{
    // keys
    pub party_id: i64,
    // fields
    pub manifest_company_name: Option<String>,
    pub manifest_company_title: Option<String>,
    pub manifest_logo_url: Option<String>,
    pub manifest_policies: Option<String>
}

