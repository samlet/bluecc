table! {
    api_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Bytea,
        name -> Varchar,
        created_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
        revoked -> Bool,
    }
}

table! {
    books (id) {
        id -> Int8,
        title -> Varchar,
        author -> Varchar,
        status -> Varchar,
    }
}

table! {
    crate_owners (crate_id, owner_id, owner_kind) {
        crate_id -> Int4,
        owner_id -> Int4,
        created_at -> Timestamp,
        created_by -> Nullable<Int4>,
        deleted -> Bool,
        updated_at -> Timestamp,
        owner_kind -> Int4,
        email_notifications -> Bool,
    }
}

table! {
    crates (id) {
        id -> Int4,
        name -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        downloads -> Int4,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        documentation -> Nullable<Varchar>,
        readme -> Nullable<Varchar>,
        repository -> Nullable<Varchar>,
        max_upload_size -> Nullable<Int4>,
    }
}

table! {
    emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        verified -> Bool,
        token -> Text,
        token_generated_at -> Nullable<Timestamp>,
    }
}

table! {
    follows (user_id, crate_id) {
        user_id -> Int4,
        crate_id -> Int4,
    }
}

table! {
    packages (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        gh_access_token -> Varchar,
        gh_login -> Varchar,
        name -> Nullable<Varchar>,
        gh_avatar -> Nullable<Varchar>,
        gh_id -> Int4,
        account_lock_reason -> Nullable<Varchar>,
        account_lock_until -> Nullable<Timestamp>,
    }
}

joinable!(api_tokens -> users (user_id));
joinable!(crate_owners -> crates (crate_id));
joinable!(crate_owners -> users (created_by));
joinable!(emails -> users (user_id));
joinable!(follows -> crates (crate_id));
joinable!(follows -> users (user_id));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    books,
    crate_owners,
    crates,
    emails,
    follows,
    packages,
    posts,
    users,
);
