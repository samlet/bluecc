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

joinable!(protected_view -> security_group (group_id));
joinable!(security_group_permission -> security_group (group_id));
joinable!(security_group_permission -> security_permission (permission_id));
joinable!(user_login_history -> user_login (user_login_id));
joinable!(user_login_password_history -> user_login (user_login_id));
joinable!(user_login_security_group -> security_group (group_id));
joinable!(user_login_security_group -> user_login (user_login_id));
joinable!(user_login_session -> user_login (user_login_id));

allow_tables_to_appear_in_same_query!(
    protected_view,
    security_group,
    security_group_permission,
    security_permission,
    tarpitted_login_view,
    user_login,
    user_login_history,
    user_login_password_history,
    user_login_security_group,
    user_login_session,
    x509_issuer_provision,
);
