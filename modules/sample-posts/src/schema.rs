table! {
    books (id) {
        id -> Unsigned<Bigint>,
        title -> Nullable<Varchar>,
        author -> Varchar,
        status -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    posts,
);
