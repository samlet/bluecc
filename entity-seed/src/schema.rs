table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    addendum (addendum_id) {
        agreement_id -> Nullable<Int8>,
        agreement_item_seq_id -> Nullable<Int8>,
        addendum_creation_date -> Nullable<Timestamptz>,
        addendum_effective_date -> Nullable<Timestamptz>,
        addendum_text -> Nullable<Varchar>,
        addendum_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    address_match_map (map_key, map_value) {
        sequence_num -> Nullable<Int8>,
        map_key -> Int8,
        map_value -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    affiliate (party_id) {
        affiliate_name -> Nullable<Varchar>,
        affiliate_description -> Nullable<Varchar>,
        year_established -> Nullable<Varchar>,
        site_type -> Nullable<Varchar>,
        site_page_views -> Nullable<Varchar>,
        site_visitors -> Nullable<Varchar>,
        date_time_created -> Nullable<Timestamptz>,
        date_time_approved -> Nullable<Timestamptz>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement (agreement_id) {
        product_id -> Nullable<Int8>,
        party_id_from -> Nullable<Int8>,
        party_id_to -> Nullable<Int8>,
        role_type_id_from -> Nullable<Int8>,
        role_type_id_to -> Nullable<Int8>,
        agreement_type_id -> Nullable<Int8>,
        agreement_date -> Nullable<Timestamptz>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        text_data -> Nullable<Text>,
        agreement_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_attribute (agreement_id, attr_name) {
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        agreement_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_content (content_id, agreement_id, agreement_item_seq_id, agreement_content_type_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        agreement_content_type_id -> Int8,
        content_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_content_type (agreement_content_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        agreement_content_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_employment_appl (agreement_id, agreement_item_seq_id, party_id_to, party_id_from, role_type_id_to, role_type_id_from, from_date) {
        agreement_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        party_id_from -> Int8,
        party_id_to -> Int8,
        role_type_id_from -> Int8,
        role_type_id_to -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_facility_appl (agreement_id, agreement_item_seq_id, facility_id) {
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        facility_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_geographical_applic (agreement_id, agreement_item_seq_id, geo_id) {
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        geo_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_item (agreement_id, agreement_item_seq_id) {
        agreement_item_type_id -> Nullable<Int8>,
        currency_uom_id -> Nullable<Int8>,
        agreement_text -> Nullable<Text>,
        agreement_image -> Nullable<Bytea>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_item_attribute (agreement_id, agreement_item_seq_id, attr_name) {
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_item_type (agreement_item_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        agreement_item_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_item_type_attr (agreement_item_type_id, attr_name) {
        description -> Nullable<Varchar>,
        agreement_item_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_party_applic (agreement_id, agreement_item_seq_id, party_id) {
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_product_appl (agreement_id, agreement_item_seq_id, product_id) {
        price -> Nullable<Numeric>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        product_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_promo_appl (agreement_id, agreement_item_seq_id, product_promo_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Int8>,
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        product_promo_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_role (agreement_id, party_id, role_type_id) {
        agreement_id -> Int8,
        party_id -> Int8,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_term (agreement_term_id) {
        term_type_id -> Nullable<Int8>,
        agreement_id -> Nullable<Int8>,
        agreement_item_seq_id -> Nullable<Int8>,
        invoice_item_type_id -> Nullable<Int8>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        term_value -> Nullable<Numeric>,
        term_days -> Nullable<Int8>,
        text_value -> Nullable<Varchar>,
        min_quantity -> Nullable<Numeric>,
        max_quantity -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        agreement_term_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_term_attribute (agreement_term_id, attr_name) {
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        agreement_term_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_type (agreement_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        agreement_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_type_attr (agreement_type_id, attr_name) {
        description -> Nullable<Varchar>,
        agreement_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    agreement_work_effort_applic (agreement_id, agreement_item_seq_id, work_effort_id) {
        agreement_id -> Int8,
        agreement_item_seq_id -> Int8,
        work_effort_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    books (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        author -> Varchar,
        status -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    comm_content_assoc_type (comm_content_assoc_type_id) {
        description -> Nullable<Varchar>,
        comm_content_assoc_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    comm_event_content_assoc (content_id, communication_event_id, from_date) {
        comm_content_assoc_type_id -> Nullable<Int8>,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Int8>,
        content_id -> Int8,
        communication_event_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    comments (id) {
        id -> Int4,
        user_id -> Int4,
        post_id -> Int4,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event (communication_event_id) {
        communication_event_type_id -> Nullable<Int8>,
        orig_comm_event_id -> Nullable<Int8>,
        parent_comm_event_id -> Nullable<Int8>,
        status_id -> Nullable<Int8>,
        contact_mech_type_id -> Nullable<Int8>,
        contact_mech_id_from -> Nullable<Int8>,
        contact_mech_id_to -> Nullable<Int8>,
        role_type_id_from -> Nullable<Int8>,
        role_type_id_to -> Nullable<Int8>,
        party_id_from -> Nullable<Int8>,
        party_id_to -> Nullable<Int8>,
        entry_date -> Nullable<Timestamptz>,
        datetime_started -> Nullable<Timestamptz>,
        datetime_ended -> Nullable<Timestamptz>,
        subject -> Nullable<Varchar>,
        content_mime_type_id -> Nullable<Int8>,
        content -> Nullable<Text>,
        note -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Int8>,
        contact_list_id -> Nullable<Int8>,
        header_string -> Nullable<Text>,
        from_string -> Nullable<Text>,
        to_string -> Nullable<Text>,
        cc_string -> Nullable<Text>,
        bcc_string -> Nullable<Text>,
        message_id -> Nullable<Varchar>,
        communication_event_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event_product (product_id, communication_event_id) {
        product_id -> Int8,
        communication_event_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event_prp_typ (communication_event_prp_typ_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        communication_event_prp_typ_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event_purpose (communication_event_prp_typ_id, communication_event_id) {
        description -> Nullable<Varchar>,
        communication_event_prp_typ_id -> Int8,
        communication_event_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event_role (communication_event_id, party_id, role_type_id) {
        contact_mech_id -> Nullable<Int8>,
        status_id -> Nullable<Int8>,
        communication_event_id -> Int8,
        party_id -> Int8,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    communication_event_type (communication_event_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        contact_mech_type_id -> Nullable<Int8>,
        communication_event_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech (contact_mech_id) {
        contact_mech_type_id -> Nullable<Int8>,
        info_string -> Nullable<Varchar>,
        contact_mech_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_attribute (contact_mech_id, attr_name) {
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        contact_mech_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_link (contact_mech_id_from, contact_mech_id_to) {
        contact_mech_id_from -> Int8,
        contact_mech_id_to -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_purpose_type (contact_mech_purpose_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        contact_mech_purpose_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_type (contact_mech_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        contact_mech_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_type_attr (contact_mech_type_id, attr_name) {
        description -> Nullable<Varchar>,
        contact_mech_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    contact_mech_type_purpose (contact_mech_type_id, contact_mech_purpose_type_id) {
        contact_mech_type_id -> Int8,
        contact_mech_purpose_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    email_address_verification (email_address) {
        verify_hash -> Nullable<Varchar>,
        expire_date -> Nullable<Timestamptz>,
        email_address -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example (example_id) {
        example_type_id -> Nullable<Int8>,
        status_id -> Nullable<Int8>,
        example_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        comments -> Nullable<Varchar>,
        example_size -> Nullable<Int8>,
        example_date -> Nullable<Timestamptz>,
        another_date -> Nullable<Timestamptz>,
        another_text -> Nullable<Varchar>,
        example_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_feature (example_feature_id) {
        feature_source_enum_id -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        example_feature_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_feature_appl (example_id, example_feature_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        example_feature_appl_type_id -> Nullable<Int8>,
        sequence_num -> Nullable<Int8>,
        example_id -> Int8,
        example_feature_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_feature_appl_type (example_feature_appl_type_id) {
        parent_type_id -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        example_feature_appl_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_item (example_id, example_item_seq_id) {
        description -> Nullable<Varchar>,
        amount -> Nullable<Float8>,
        amount_uom_id -> Nullable<Int8>,
        example_id -> Int8,
        example_item_seq_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_status (example_id, status_date) {
        status_end_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Int8>,
        status_id -> Nullable<Int8>,
        example_id -> Int8,
        status_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    example_type (example_type_id) {
        parent_type_id -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        example_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    ftp_address (contact_mech_id) {
        hostname -> Nullable<Varchar>,
        port -> Nullable<Int8>,
        username -> Nullable<Varchar>,
        ftp_password -> Nullable<Varchar>,
        binary_transfer -> Nullable<Bool>,
        file_path -> Nullable<Varchar>,
        zip_file -> Nullable<Bool>,
        passive_mode -> Nullable<Bool>,
        default_timeout -> Nullable<Int8>,
        contact_mech_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    mnemonics (id) {
        id -> Int4,
        path -> Varchar,
        num_value -> Int4,
        string_value -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    need_type (need_type_id) {
        description -> Nullable<Varchar>,
        need_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party (party_id) {
        party_type_id -> Nullable<Int8>,
        external_id -> Nullable<Int8>,
        preferred_currency_uom_id -> Nullable<Int8>,
        description -> Nullable<Text>,
        status_id -> Nullable<Int8>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Int8>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Int8>,
        data_source_id -> Nullable<Int8>,
        is_unread -> Nullable<Bool>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_attribute (party_id, attr_name) {
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        party_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_carrier_account (party_id, carrier_party_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        account_number -> Nullable<Int8>,
        party_id -> Int8,
        carrier_party_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_classification (party_id, party_classification_group_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        party_id -> Int8,
        party_classification_group_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_classification_group (party_classification_group_id) {
        party_classification_type_id -> Nullable<Int8>,
        parent_group_id -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        party_classification_group_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_classification_type (party_classification_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        party_classification_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_contact_mech (party_id, contact_mech_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        role_type_id -> Nullable<Int8>,
        allow_solicitation -> Nullable<Bool>,
        extension -> Nullable<Varchar>,
        verified -> Nullable<Bool>,
        comments -> Nullable<Varchar>,
        years_with_contact_mech -> Nullable<Int8>,
        months_with_contact_mech -> Nullable<Int8>,
        party_id -> Int8,
        contact_mech_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_contact_mech_purpose (party_id, contact_mech_id, contact_mech_purpose_type_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        party_id -> Int8,
        contact_mech_id -> Int8,
        contact_mech_purpose_type_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_content (party_id, content_id, party_content_type_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        party_id -> Int8,
        content_id -> Int8,
        party_content_type_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_content_type (party_content_type_id) {
        parent_type_id -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        party_content_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_data_source (party_id, data_source_id, from_date) {
        visit_id -> Nullable<Int8>,
        comments -> Nullable<Varchar>,
        is_create -> Nullable<Bool>,
        party_id -> Int8,
        data_source_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_geo_point (party_id, geo_point_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        party_id -> Int8,
        geo_point_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_group (party_id) {
        group_name -> Nullable<Varchar>,
        group_name_local -> Nullable<Varchar>,
        office_site_name -> Nullable<Varchar>,
        annual_revenue -> Nullable<Numeric>,
        num_employees -> Nullable<Int8>,
        ticker_symbol -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        logo_image_url -> Nullable<Varchar>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_ics_avs_override (party_id) {
        avs_decline_string -> Nullable<Varchar>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_identification (party_id, party_identification_type_id) {
        id_value -> Nullable<Int8>,
        party_id -> Int8,
        party_identification_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_identification_type (party_identification_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        party_identification_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_invitation (party_invitation_id) {
        party_id_from -> Nullable<Int8>,
        party_id -> Nullable<Int8>,
        to_name -> Nullable<Varchar>,
        email_address -> Nullable<Varchar>,
        status_id -> Nullable<Int8>,
        last_invite_date -> Nullable<Timestamptz>,
        party_invitation_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_invitation_group_assoc (party_invitation_id, party_id_to) {
        party_invitation_id -> Int8,
        party_id_to -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_invitation_role_assoc (party_invitation_id, role_type_id) {
        party_invitation_id -> Int8,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_name_history (party_id, change_date) {
        group_name -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        personal_title -> Nullable<Varchar>,
        suffix -> Nullable<Varchar>,
        party_id -> Int8,
        change_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_need (party_need_id, party_id, role_type_id) {
        party_type_id -> Nullable<Int8>,
        need_type_id -> Nullable<Int8>,
        communication_event_id -> Nullable<Int8>,
        product_id -> Nullable<Int8>,
        product_category_id -> Nullable<Int8>,
        visit_id -> Nullable<Int8>,
        datetime_recorded -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        party_need_id -> Int8,
        party_id -> Int8,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_note (party_id, note_id) {
        party_id -> Int8,
        note_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_profile_default (party_id, product_store_id) {
        default_ship_addr -> Nullable<Int8>,
        default_bill_addr -> Nullable<Int8>,
        default_pay_meth -> Nullable<Int8>,
        default_ship_meth -> Nullable<Int8>,
        party_id -> Int8,
        product_store_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_relationship (party_id_from, party_id_to, role_type_id_from, role_type_id_to, from_date) {
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Int8>,
        relationship_name -> Nullable<Varchar>,
        security_group_id -> Nullable<Int8>,
        priority_type_id -> Nullable<Int8>,
        party_relationship_type_id -> Nullable<Int8>,
        permissions_enum_id -> Nullable<Int8>,
        position_title -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        party_id_from -> Int8,
        party_id_to -> Int8,
        role_type_id_from -> Int8,
        role_type_id_to -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_relationship_type (party_relationship_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        party_relationship_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        role_type_id_valid_from -> Nullable<Int8>,
        role_type_id_valid_to -> Nullable<Int8>,
        party_relationship_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_role (party_id, role_type_id) {
        party_id -> Int8,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_status (status_id, party_id, status_date) {
        change_by_user_login_id -> Nullable<Int8>,
        status_id -> Int8,
        party_id -> Int8,
        status_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_type (party_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        party_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    party_type_attr (party_type_id, attr_name) {
        description -> Nullable<Varchar>,
        party_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    person (party_id) {
        salutation -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        personal_title -> Nullable<Varchar>,
        suffix -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        first_name_local -> Nullable<Varchar>,
        middle_name_local -> Nullable<Varchar>,
        last_name_local -> Nullable<Varchar>,
        other_local -> Nullable<Varchar>,
        member_id -> Nullable<Int8>,
        gender -> Nullable<Bool>,
        birth_date -> Nullable<Date>,
        deceased_date -> Nullable<Date>,
        height -> Nullable<Numeric>,
        weight -> Nullable<Numeric>,
        mothers_maiden_name -> Nullable<Varchar>,
        old_marital_status -> Nullable<Bool>,
        marital_status_enum_id -> Nullable<Int8>,
        social_security_number -> Nullable<Varchar>,
        passport_number -> Nullable<Varchar>,
        passport_expire_date -> Nullable<Date>,
        total_years_work_experience -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        employment_status_enum_id -> Nullable<Int8>,
        residence_status_enum_id -> Nullable<Int8>,
        occupation -> Nullable<Varchar>,
        years_with_employer -> Nullable<Int8>,
        months_with_employer -> Nullable<Int8>,
        existing_customer -> Nullable<Bool>,
        card_id -> Nullable<Int8>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    postal_address (contact_mech_id) {
        to_name -> Nullable<Varchar>,
        attn_name -> Nullable<Varchar>,
        address_1 -> Nullable<Varchar>,
        address_2 -> Nullable<Varchar>,
        house_number -> Nullable<Int8>,
        house_number_ext -> Nullable<Varchar>,
        directions -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        city_geo_id -> Nullable<Int8>,
        postal_code -> Nullable<Varchar>,
        postal_code_ext -> Nullable<Varchar>,
        country_geo_id -> Nullable<Int8>,
        state_province_geo_id -> Nullable<Int8>,
        county_geo_id -> Nullable<Int8>,
        municipality_geo_id -> Nullable<Int8>,
        postal_code_geo_id -> Nullable<Int8>,
        geo_point_id -> Nullable<Int8>,
        contact_mech_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    postal_address_boundary (contact_mech_id, geo_id) {
        contact_mech_id -> Int8,
        geo_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    priority_type (priority_type_id) {
        description -> Nullable<Varchar>,
        priority_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    protected_view (group_id, view_name_id) {
        max_hits -> Nullable<Int8>,
        max_hits_duration -> Nullable<Int8>,
        tarpit_duration -> Nullable<Int8>,
        group_id -> Int8,
        view_name_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    role_type (role_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        role_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    role_type_attr (role_type_id, attr_name) {
        description -> Nullable<Varchar>,
        role_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    security_group (group_id) {
        group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        group_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    security_group_permission (group_id, permission_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        group_id -> Int8,
        permission_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    security_permission (permission_id) {
        description -> Nullable<Varchar>,
        permission_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    tarpitted_login_view (view_name_id, user_login_id) {
        tarpit_release_date_time -> Nullable<Int8>,
        view_name_id -> Int8,
        user_login_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    telecom_number (contact_mech_id) {
        country_code -> Nullable<Varchar>,
        area_code -> Nullable<Varchar>,
        contact_number -> Nullable<Varchar>,
        ask_for_name -> Nullable<Varchar>,
        contact_mech_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    term_type (term_type_id) {
        parent_type_id -> Nullable<Int8>,
        has_table -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        term_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    term_type_attr (term_type_id, attr_name) {
        description -> Nullable<Varchar>,
        term_type_id -> Int8,
        attr_name -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    test_person (party_id) {
        salutation -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        personal_title -> Nullable<Varchar>,
        suffix -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        first_name_local -> Nullable<Varchar>,
        middle_name_local -> Nullable<Varchar>,
        last_name_local -> Nullable<Varchar>,
        other_local -> Nullable<Varchar>,
        member_id -> Nullable<Int8>,
        gender -> Nullable<Bool>,
        birth_date -> Nullable<Date>,
        deceased_date -> Nullable<Date>,
        height -> Nullable<Numeric>,
        weight -> Nullable<Numeric>,
        mothers_maiden_name -> Nullable<Varchar>,
        old_marital_status -> Nullable<Bool>,
        marital_status_enum_id -> Nullable<Int8>,
        social_security_number -> Nullable<Varchar>,
        passport_number -> Nullable<Varchar>,
        passport_expire_date -> Nullable<Date>,
        total_years_work_experience -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        employment_status_enum_id -> Nullable<Int8>,
        residence_status_enum_id -> Nullable<Int8>,
        occupation -> Nullable<Varchar>,
        years_with_employer -> Nullable<Int8>,
        months_with_employer -> Nullable<Int8>,
        existing_customer -> Nullable<Bool>,
        card_id -> Nullable<Int8>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    user_login (user_login_id) {
        current_password -> Nullable<Varchar>,
        password_hint -> Nullable<Varchar>,
        is_system -> Nullable<Bool>,
        enabled -> Nullable<Bool>,
        has_logged_out -> Nullable<Bool>,
        require_password_change -> Nullable<Bool>,
        last_currency_uom -> Nullable<Int8>,
        last_locale -> Nullable<Varchar>,
        last_time_zone -> Nullable<Int8>,
        disabled_date_time -> Nullable<Timestamptz>,
        successive_failed_logins -> Nullable<Int8>,
        external_auth_id -> Nullable<Int8>,
        user_ldap_dn -> Nullable<Int8>,
        disabled_by -> Nullable<Int8>,
        user_login_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    user_login_history (user_login_id, from_date) {
        visit_id -> Nullable<Int8>,
        thru_date -> Nullable<Timestamptz>,
        password_used -> Nullable<Varchar>,
        successful_login -> Nullable<Bool>,
        origin_user_login_id -> Nullable<Int8>,
        user_login_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    user_login_password_history (user_login_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        current_password -> Nullable<Varchar>,
        user_login_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    user_login_security_group (user_login_id, group_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        user_login_id -> Int8,
        group_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    user_login_session (user_login_id) {
        saved_date -> Nullable<Timestamptz>,
        session_data -> Nullable<Text>,
        user_login_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    valid_contact_mech_role (role_type_id, contact_mech_type_id) {
        role_type_id -> Int8,
        contact_mech_type_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    vendor (party_id) {
        manifest_company_name -> Nullable<Varchar>,
        manifest_company_title -> Nullable<Varchar>,
        manifest_logo_url -> Nullable<Varchar>,
        manifest_policies -> Nullable<Text>,
        party_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::{TsVector as Tsvector};
    use bigdecimal::BigDecimal;

    x509_issuer_provision (cert_provision_id) {
        common_name -> Nullable<Varchar>,
        organizational_unit -> Nullable<Varchar>,
        organization_name -> Nullable<Varchar>,
        city_locality -> Nullable<Varchar>,
        state_province -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        serial_number -> Nullable<Varchar>,
        cert_provision_id -> Int8,
    }
}

joinable!(addendum -> agreement (agreement_id));
joinable!(affiliate -> party (party_id));
joinable!(affiliate -> party_group (party_id));
joinable!(agreement -> agreement_type (agreement_type_id));
joinable!(agreement -> party (party_id_from));
joinable!(agreement -> role_type (role_type_id_from));
joinable!(agreement_attribute -> agreement (agreement_id));
joinable!(agreement_content -> agreement (agreement_id));
joinable!(agreement_content -> agreement_content_type (agreement_content_type_id));
joinable!(agreement_facility_appl -> agreement (agreement_id));
joinable!(agreement_geographical_applic -> agreement (agreement_id));
joinable!(agreement_item -> agreement (agreement_id));
joinable!(agreement_item -> agreement_item_type (agreement_item_type_id));
joinable!(agreement_item_type_attr -> agreement_item_type (agreement_item_type_id));
joinable!(agreement_party_applic -> agreement (agreement_id));
joinable!(agreement_party_applic -> party (party_id));
joinable!(agreement_product_appl -> agreement (agreement_id));
joinable!(agreement_promo_appl -> agreement (agreement_id));
joinable!(agreement_role -> agreement (agreement_id));
joinable!(agreement_role -> party (party_id));
joinable!(agreement_role -> role_type (role_type_id));
joinable!(agreement_term -> agreement (agreement_id));
joinable!(agreement_term -> term_type (term_type_id));
joinable!(agreement_term_attribute -> agreement_term (agreement_term_id));
joinable!(agreement_type_attr -> agreement_type (agreement_type_id));
joinable!(agreement_work_effort_applic -> agreement (agreement_id));
joinable!(comm_event_content_assoc -> comm_content_assoc_type (comm_content_assoc_type_id));
joinable!(comm_event_content_assoc -> communication_event (communication_event_id));
joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(communication_event -> communication_event_type (communication_event_type_id));
joinable!(communication_event -> contact_mech (contact_mech_id_from));
joinable!(communication_event -> contact_mech_type (contact_mech_type_id));
joinable!(communication_event -> party (party_id_to));
joinable!(communication_event -> role_type (role_type_id_to));
joinable!(communication_event_product -> communication_event (communication_event_id));
joinable!(communication_event_purpose -> communication_event (communication_event_id));
joinable!(communication_event_purpose -> communication_event_prp_typ (communication_event_prp_typ_id));
joinable!(communication_event_role -> communication_event (communication_event_id));
joinable!(communication_event_role -> contact_mech (contact_mech_id));
joinable!(communication_event_role -> party (party_id));
joinable!(communication_event_role -> role_type (role_type_id));
joinable!(communication_event_type -> contact_mech_type (contact_mech_type_id));
joinable!(contact_mech -> contact_mech_type (contact_mech_type_id));
joinable!(contact_mech_attribute -> contact_mech (contact_mech_id));
joinable!(contact_mech_link -> contact_mech (contact_mech_id_from));
joinable!(contact_mech_type_attr -> contact_mech_type (contact_mech_type_id));
joinable!(contact_mech_type_purpose -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(contact_mech_type_purpose -> contact_mech_type (contact_mech_type_id));
joinable!(example -> example_type (example_type_id));
joinable!(example_feature_appl -> example (example_id));
joinable!(example_feature_appl -> example_feature (example_feature_id));
joinable!(example_feature_appl -> example_feature_appl_type (example_feature_appl_type_id));
joinable!(example_item -> example (example_id));
joinable!(example_status -> example (example_id));
joinable!(ftp_address -> contact_mech (contact_mech_id));
joinable!(party -> party_type (party_type_id));
joinable!(party_attribute -> party (party_id));
joinable!(party_carrier_account -> party (party_id));
joinable!(party_classification -> party (party_id));
joinable!(party_classification -> party_classification_group (party_classification_group_id));
joinable!(party_classification_group -> party_classification_type (party_classification_type_id));
joinable!(party_contact_mech -> contact_mech (contact_mech_id));
joinable!(party_contact_mech -> party (party_id));
joinable!(party_contact_mech -> party_group (party_id));
joinable!(party_contact_mech -> person (party_id));
joinable!(party_contact_mech -> postal_address (contact_mech_id));
joinable!(party_contact_mech -> role_type (role_type_id));
joinable!(party_contact_mech -> telecom_number (contact_mech_id));
joinable!(party_contact_mech_purpose -> contact_mech (contact_mech_id));
joinable!(party_contact_mech_purpose -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(party_contact_mech_purpose -> party (party_id));
joinable!(party_contact_mech_purpose -> party_group (party_id));
joinable!(party_contact_mech_purpose -> person (party_id));
joinable!(party_contact_mech_purpose -> postal_address (contact_mech_id));
joinable!(party_contact_mech_purpose -> telecom_number (contact_mech_id));
joinable!(party_content -> party (party_id));
joinable!(party_content -> party_content_type (party_content_type_id));
joinable!(party_data_source -> party (party_id));
joinable!(party_geo_point -> party (party_id));
joinable!(party_group -> party (party_id));
joinable!(party_ics_avs_override -> party (party_id));
joinable!(party_identification -> party (party_id));
joinable!(party_identification -> party_identification_type (party_identification_type_id));
joinable!(party_invitation -> party (party_id_from));
joinable!(party_invitation_group_assoc -> party (party_id_to));
joinable!(party_invitation_group_assoc -> party_group (party_id_to));
joinable!(party_invitation_group_assoc -> party_invitation (party_invitation_id));
joinable!(party_invitation_role_assoc -> party_invitation (party_invitation_id));
joinable!(party_invitation_role_assoc -> role_type (role_type_id));
joinable!(party_name_history -> party (party_id));
joinable!(party_need -> communication_event (communication_event_id));
joinable!(party_need -> need_type (need_type_id));
joinable!(party_need -> party (party_id));
joinable!(party_need -> party_type (party_type_id));
joinable!(party_need -> role_type (role_type_id));
joinable!(party_note -> party (party_id));
joinable!(party_profile_default -> party (party_id));
joinable!(party_relationship -> party (party_id_from));
joinable!(party_relationship -> party_relationship_type (party_relationship_type_id));
joinable!(party_relationship -> priority_type (priority_type_id));
joinable!(party_relationship -> role_type (role_type_id_from));
joinable!(party_relationship_type -> role_type (role_type_id_valid_from));
joinable!(party_role -> party (party_id));
joinable!(party_role -> role_type (role_type_id));
joinable!(party_status -> party (party_id));
joinable!(party_type_attr -> party_type (party_type_id));
joinable!(person -> party (party_id));
joinable!(postal_address -> contact_mech (contact_mech_id));
joinable!(postal_address_boundary -> postal_address (contact_mech_id));
joinable!(posts -> users (user_id));
joinable!(protected_view -> security_group (group_id));
joinable!(role_type_attr -> role_type (role_type_id));
joinable!(security_group_permission -> security_group (group_id));
joinable!(security_group_permission -> security_permission (permission_id));
joinable!(telecom_number -> contact_mech (contact_mech_id));
joinable!(term_type_attr -> term_type (term_type_id));
joinable!(user_login_history -> user_login (user_login_id));
joinable!(user_login_password_history -> user_login (user_login_id));
joinable!(user_login_security_group -> security_group (group_id));
joinable!(user_login_security_group -> user_login (user_login_id));
joinable!(user_login_session -> user_login (user_login_id));
joinable!(valid_contact_mech_role -> contact_mech_type (contact_mech_type_id));
joinable!(valid_contact_mech_role -> role_type (role_type_id));
joinable!(vendor -> party (party_id));

allow_tables_to_appear_in_same_query!(
    addendum,
    address_match_map,
    affiliate,
    agreement,
    agreement_attribute,
    agreement_content,
    agreement_content_type,
    agreement_employment_appl,
    agreement_facility_appl,
    agreement_geographical_applic,
    agreement_item,
    agreement_item_attribute,
    agreement_item_type,
    agreement_item_type_attr,
    agreement_party_applic,
    agreement_product_appl,
    agreement_promo_appl,
    agreement_role,
    agreement_term,
    agreement_term_attribute,
    agreement_type,
    agreement_type_attr,
    agreement_work_effort_applic,
    books,
    comm_content_assoc_type,
    comm_event_content_assoc,
    comments,
    communication_event,
    communication_event_product,
    communication_event_prp_typ,
    communication_event_purpose,
    communication_event_role,
    communication_event_type,
    contact_mech,
    contact_mech_attribute,
    contact_mech_link,
    contact_mech_purpose_type,
    contact_mech_type,
    contact_mech_type_attr,
    contact_mech_type_purpose,
    email_address_verification,
    example,
    example_feature,
    example_feature_appl,
    example_feature_appl_type,
    example_item,
    example_status,
    example_type,
    ftp_address,
    mnemonics,
    need_type,
    party,
    party_attribute,
    party_carrier_account,
    party_classification,
    party_classification_group,
    party_classification_type,
    party_contact_mech,
    party_contact_mech_purpose,
    party_content,
    party_content_type,
    party_data_source,
    party_geo_point,
    party_group,
    party_ics_avs_override,
    party_identification,
    party_identification_type,
    party_invitation,
    party_invitation_group_assoc,
    party_invitation_role_assoc,
    party_name_history,
    party_need,
    party_note,
    party_profile_default,
    party_relationship,
    party_relationship_type,
    party_role,
    party_status,
    party_type,
    party_type_attr,
    person,
    postal_address,
    postal_address_boundary,
    posts,
    priority_type,
    protected_view,
    role_type,
    role_type_attr,
    security_group,
    security_group_permission,
    security_permission,
    tarpitted_login_view,
    telecom_number,
    term_type,
    term_type_attr,
    test_person,
    user_login,
    user_login_history,
    user_login_password_history,
    user_login_security_group,
    user_login_session,
    users,
    valid_contact_mech_role,
    vendor,
    x509_issuer_provision,
);
