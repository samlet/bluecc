create table crates
(
    id                       SERIAL PRIMARY KEY,
    name                     varchar                             not null,
    updated_at               timestamp default CURRENT_TIMESTAMP not null,
    created_at               timestamp default CURRENT_TIMESTAMP not null,
    downloads                integer   default 0                 not null,
    description              varchar,
    homepage                 varchar,
    documentation            varchar,
    readme                   varchar,
--    textsearchable_index_col tsvector                            not null,
    repository               varchar,
    max_upload_size          integer
);

create index index_crate_updated_at
    on crates (updated_at);

create index index_crate_created_at
    on crates (created_at);

create index index_crates_name_ordering
    on crates (name);

-- #############

--CREATE TABLE users (
--            id              SERIAL PRIMARY KEY,
--            email           VARCHAR NOT NULL UNIQUE,
--            gh_access_token VARCHAR NOT NULL,
--            api_token       VARCHAR NOT NULL
--        );
create table users
(
    id                  serial  not null
        constraint users_pkey
            primary key,
    gh_access_token     varchar not null,
    gh_login            varchar not null,
    name                varchar,
    gh_avatar           varchar,
    gh_id               integer not null,
    account_lock_reason varchar,
    account_lock_until  timestamp
);


CREATE TABLE packages (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL UNIQUE,
            user_id         INTEGER NOT NULL
        );

--CREATE TABLE follows (
--            user_id          INTEGER NOT NULL,
--            crate_id         INTEGER NOT NULL
--        );

--CREATE TABLE crate_owners (
--            id               SERIAL PRIMARY KEY,
--            crate_id         INTEGER NOT NULL,
--            user_id          INTEGER NOT NULL,
--            created_at       TIMESTAMP NOT NULL,
--            created_by       INTEGER
--        );

create table crate_owners
(
    crate_id            integer                             not null
        constraint fk_crate_owners_crate_id
            references crates
            on delete cascade,
    owner_id            integer                             not null,
    created_at          timestamp default CURRENT_TIMESTAMP not null,
    created_by          integer
        constraint fk_crate_owners_created_by
            references users,
    deleted             boolean   default false             not null,
    updated_at          timestamp default CURRENT_TIMESTAMP not null,
    owner_kind          integer                             not null,
    email_notifications boolean   default true              not null,
    constraint crate_owners_pkey
        primary key (crate_id, owner_id, owner_kind)
);

--CREATE table emails (
--    id          SERIAL PRIMARY KEY,
--    user_id     INTEGER NOT NULL UNIQUE,
--    email       VARCHAR NOT NULL,
--    verified    BOOLEAN DEFAULT false NOT NULL
--);
create table emails
(
    id                 serial                            not null
        constraint emails_pkey
            primary key,
    user_id            integer                           not null
        constraint emails_user_id_key
            unique
        constraint fk_emails_user_id
            references users,
    email              varchar                           not null,
    verified           boolean default false             not null,
    token              text     not null,
    token_generated_at timestamp
);
--ALTER TABLE emails ADD COLUMN token TEXT NOT NULL DEFAULT random_string(26);

--CREATE table tokens (
--    id          SERIAL PRIMARY KEY,
--    email_id    INTEGER NOT NULL UNIQUE REFERENCES emails,
--    token       VARCHAR NOT NULL,
--    created_at  TIMESTAMP NOT NULL DEFAULT now()
--);
create table api_tokens
(
    id           serial                  not null
        constraint api_tokens_pkey
            primary key,
    user_id      integer                 not null
        constraint api_tokens_user_id_fkey
            references users,
    token        bytea                   not null,
    name         varchar                 not null,
    created_at   timestamp default now() not null,
    last_used_at timestamp,
    revoked      boolean   default false not null
);
