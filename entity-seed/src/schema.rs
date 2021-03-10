table! {
    books (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        author -> Varchar,
        status -> Nullable<Varchar>,
    }
}

table! {
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
    mnemonics (id) {
        id -> Int4,
        path -> Varchar,
        num_value -> Int4,
        string_value -> Varchar,
        description -> Nullable<Text>,
    }
}

table! {
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
    protected_view (group_id, view_name_id) {
        max_hits -> Nullable<Numeric>,
        max_hits_duration -> Nullable<Numeric>,
        tarpit_duration -> Nullable<Numeric>,
        group_id -> Int8,
        view_name_id -> Int8,
    }
}

table! {
    security_group (group_id) {
        group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        group_id -> Int8,
    }
}

table! {
    security_group_permission (group_id, permission_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        group_id -> Int8,
        permission_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    security_permission (permission_id) {
        description -> Nullable<Varchar>,
        permission_id -> Int8,
    }
}

table! {
    tarpitted_login_view (view_name_id, user_login_id) {
        tarpit_release_date_time -> Nullable<Numeric>,
        view_name_id -> Int8,
        user_login_id -> Int8,
    }
}

table! {
    user_login (user_login_id) {
        current_password -> Nullable<Varchar>,
        password_hint -> Nullable<Varchar>,
        is_system -> Nullable<Bpchar>,
        enabled -> Nullable<Bpchar>,
        has_logged_out -> Nullable<Bpchar>,
        require_password_change -> Nullable<Bpchar>,
        last_currency_uom -> Int8,
        last_locale -> Nullable<Varchar>,
        last_time_zone -> Int8,
        disabled_date_time -> Nullable<Timestamptz>,
        successive_failed_logins -> Nullable<Numeric>,
        external_auth_id -> Int8,
        user_ldap_dn -> Int8,
        disabled_by -> Int8,
        user_login_id -> Int8,
    }
}

table! {
    user_login_history (user_login_id, from_date) {
        visit_id -> Int8,
        thru_date -> Nullable<Timestamptz>,
        password_used -> Nullable<Varchar>,
        successful_login -> Nullable<Bpchar>,
        origin_user_login_id -> Int8,
        user_login_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    user_login_password_history (user_login_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        current_password -> Nullable<Varchar>,
        user_login_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    user_login_security_group (user_login_id, group_id, from_date) {
        thru_date -> Nullable<Timestamptz>,
        user_login_id -> Int8,
        group_id -> Int8,
        from_date -> Timestamptz,
    }
}

table! {
    user_login_session (user_login_id) {
        saved_date -> Nullable<Timestamptz>,
        session_data -> Nullable<Text>,
        user_login_id -> Int8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
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
    mnemonics,
    posts,
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
    users,
    x509_issuer_provision,
);
