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
        mothers_maiden_name -> Nullable<Varchar>,
        old_marital_status -> Nullable<Bool>,
        marital_status_enum_id -> Nullable<Int8>,
        social_security_number -> Nullable<Varchar>,
        passport_number -> Nullable<Varchar>,
        passport_expire_date -> Nullable<Date>,
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

joinable!(comments -> posts (post_id));
joinable!(comments -> users (user_id));
joinable!(example -> example_type (example_type_id));
joinable!(example_feature_appl -> example (example_id));
joinable!(example_feature_appl -> example_feature (example_feature_id));
joinable!(example_feature_appl -> example_feature_appl_type (example_feature_appl_type_id));
joinable!(example_item -> example (example_id));
joinable!(example_status -> example (example_id));
joinable!(posts -> users (user_id));
joinable!(protected_view -> security_group (group_id));
joinable!(security_group_permission -> security_group (group_id));
joinable!(security_group_permission -> security_permission (permission_id));
joinable!(user_login_history -> user_login (user_login_id));
joinable!(user_login_password_history -> user_login (user_login_id));
joinable!(user_login_security_group -> security_group (group_id));
joinable!(user_login_security_group -> user_login (user_login_id));
joinable!(user_login_session -> user_login (user_login_id));

allow_tables_to_appear_in_same_query!(
    books,
    comments,
    example,
    example_feature,
    example_feature_appl,
    example_feature_appl_type,
    example_item,
    example_status,
    example_type,
    mnemonics,
    posts,
    protected_view,
    security_group,
    security_group_permission,
    security_permission,
    tarpitted_login_view,
    test_person,
    user_login,
    user_login_history,
    user_login_password_history,
    user_login_security_group,
    user_login_session,
    users,
    x509_issuer_provision,
);
