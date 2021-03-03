CREATE FUNCTION canon_crate_name(text) RETURNS text AS $$
                    SELECT replace(lower($1), '-', '_')
                $$ LANGUAGE SQL
            ;

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

create unique index users_gh_id
    on users (gh_id)
    where (gh_id > 0);

create index lower_gh_login
    on users (lower(gh_login::text));

create table crates
(
    id                       serial                              not null
        constraint packages_pkey
            primary key,
    name                     varchar                             not null,
    updated_at               timestamp default CURRENT_TIMESTAMP not null,
    created_at               timestamp default CURRENT_TIMESTAMP not null,
    downloads                integer   default 0                 not null,
    description              varchar,
    homepage                 varchar,
    documentation            varchar,
    readme                   varchar,
    textsearchable_index_col tsvector                            not null,
    repository               varchar,
    max_upload_size          integer
);

create index index_crate_updated_at
    on crates (updated_at);

create index index_crate_created_at
    on crates (created_at);

create index index_crate_downloads
    on crates (downloads);

create index index_crates_name_search
    on crates using gin (textsearchable_index_col);

create unique index index_crates_name
    on crates (canon_crate_name(name::text));

create index index_crates_name_ordering
    on crates (name);

--create index index_crates_name_tgrm
--    on crates using gin (canon_crate_name(name::text) gin_trgm_ops);

--create trigger trigger_crates_set_updated_at
--    before update
--    on crates
--    for each row
--execute procedure set_updated_at_ignore_downloads();
--
--create trigger trigger_ensure_crate_name_not_reserved
--    before insert or update
--    on crates
--    for each row
--execute procedure ensure_crate_name_not_reserved();
--
--create trigger trigger_crates_tsvector_update
--    before insert or update
--        of updated_at
--    on crates
--    for each row
--execute procedure trigger_crates_name_search();

create table versions
(
    id           serial                              not null
        constraint versions_pkey
            primary key,
    crate_id     integer                             not null
        constraint fk_versions_crate_id
            references crates
            on delete cascade,
    num          varchar                             not null,
    updated_at   timestamp default CURRENT_TIMESTAMP not null,
    created_at   timestamp default CURRENT_TIMESTAMP not null,
    downloads    integer   default 0                 not null,
    features     jsonb     default '{}'::jsonb       not null,
    yanked       boolean   default false             not null,
    license      varchar,
    crate_size   integer,
    published_by integer
        constraint fk_versions_published_by
            references users,
    constraint unique_num
        unique (crate_id, num)
);

--create trigger trigger_versions_set_updated_at
--    before update
--    on versions
--    for each row
--execute procedure set_updated_at_ignore_downloads();

--create trigger touch_crate
--    before insert or update
--    on versions
--    for each row
--execute procedure touch_crate_on_version_modified();

create table metadata
(
    total_downloads bigint not null
        constraint metadata_pkey
            primary key
);

create table version_downloads
(
    version_id integer                      not null
        constraint fk_version_downloads_version_id
            references versions
            on delete cascade,
    downloads  integer default 1            not null,
    counted    integer default 0            not null,
    date       date    default CURRENT_DATE not null,
    processed  boolean default false        not null,
    constraint version_downloads_pkey
        primary key (version_id, date)
);

create index index_version_downloads_by_date
    on version_downloads using brin (date);

create table dependencies
(
    id               serial            not null
        constraint dependencies_pkey
            primary key,
    version_id       integer           not null
        constraint fk_dependencies_version_id
            references versions
            on delete cascade,
    crate_id         integer           not null
        constraint fk_dependencies_crate_id
            references crates
            on delete cascade,
    req              varchar           not null,
    optional         boolean           not null,
    default_features boolean           not null,
    features         text[]            not null,
    target           varchar,
    kind             integer default 0 not null
);

create index index_dependencies_version_id
    on dependencies (version_id);

create index index_dependencies_crate_id
    on dependencies (crate_id);

create index dependencies_crate_id_version_id_idx
    on dependencies (crate_id, version_id);

create table follows
(
    user_id  integer not null
        constraint fk_follows_user_id
            references users,
    crate_id integer not null
        constraint fk_follows_crate_id
            references crates
            on delete cascade,
    constraint follows_pkey
        primary key (user_id, crate_id)
);

create index index_follows_user_id
    on follows (user_id);

create table version_authors
(
    id         serial  not null
        constraint version_authors_pkey
            primary key,
    version_id integer not null
        constraint fk_version_authors_version_id
            references versions
            on delete cascade,
    name       varchar not null
);

create index index_version_authors_version_id
    on version_authors (version_id);

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

create unique index crate_owners_not_deleted
    on crate_owners (crate_id, owner_id, owner_kind)
    where (NOT deleted);

CREATE FUNCTION set_updated_at() RETURNS trigger AS $$
            BEGIN
                IF (NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at) THEN
                    NEW.updated_at := current_timestamp;
                END IF;
                RETURN NEW;
            END
            $$ LANGUAGE plpgsql;

create trigger trigger_crate_owners_set_updated_at
    before update
    on crate_owners
    for each row
execute procedure set_updated_at();

create table keywords
(
    id         serial                              not null
        constraint keywords_pkey
            primary key,
    keyword    text                                not null
        constraint keywords_keyword_key
            unique,
    crates_cnt integer   default 0                 not null,
    created_at timestamp default CURRENT_TIMESTAMP not null
);

create index index_keywords_crates_cnt
    on keywords (crates_cnt);

create index index_keywords_keyword
    on keywords (keyword);

create index index_keywords_lower_keyword
    on keywords (lower(keyword));

create table crates_keywords
(
    crate_id   integer not null
        constraint fk_crates_keywords_crate_id
            references crates
            on delete cascade,
    keyword_id integer not null
        constraint fk_crates_keywords_keyword_id
            references keywords,
    constraint crates_keywords_pkey
        primary key (crate_id, keyword_id)
);

create index index_crates_keywords_crate_id
    on crates_keywords (crate_id);

create index index_crates_keywords_keyword_id
    on crates_keywords (keyword_id);

CREATE FUNCTION update_keywords_crates_cnt() RETURNS trigger AS $$
            BEGIN
                IF (TG_OP = 'INSERT') THEN
                    UPDATE keywords SET crates_cnt = crates_cnt + 1 WHERE id = NEW.keyword_id;
                    return NEW;
                ELSIF (TG_OP = 'DELETE') THEN
                    UPDATE keywords SET crates_cnt = crates_cnt - 1 WHERE id = OLD.keyword_id;
                    return OLD;
                END IF;
            END
            $$ LANGUAGE plpgsql;

create trigger trigger_update_keywords_crates_cnt
    before insert or delete
    on crates_keywords
    for each row
execute procedure update_keywords_crates_cnt();

--create trigger touch_crate_on_modify_keywords
--    after insert or delete
--    on crates_keywords
--    for each row
--execute procedure touch_crate();

create table teams
(
    id        serial  not null
        constraint teams_pkey
            primary key,
    login     varchar not null
        constraint teams_login_key
            unique
        constraint teams_login_lowercase_ck
            check ((login)::text = lower((login)::text)),
    github_id integer not null
        constraint teams_github_id_key
            unique,
    name      varchar,
    avatar    varchar,
    org_id    integer
);

create table categories
(
    id          serial                                  not null
        constraint categories_pkey
            primary key,
    category    varchar                                 not null
        constraint categories_category_key
            unique,
    slug        varchar                                 not null
        constraint categories_slug_key
            unique,
    description varchar   default ''::character varying not null,
    crates_cnt  integer   default 0                     not null,
    created_at  timestamp default CURRENT_TIMESTAMP     not null
--    path        Ltree                                   not null
);

--create index path_gist_categories_idx
--    on categories using gist (path);

--create trigger set_category_path_insert
--    before insert
--    on categories
--    for each row
--execute procedure set_category_path_to_slug();
--
--create trigger set_category_path_update
--    before update
--        of slug
--    on categories
--    for each row
--execute procedure set_category_path_to_slug();

create table crates_categories
(
    crate_id    integer not null
        constraint fk_crates_categories_crate_id
            references crates
            on delete cascade,
    category_id integer not null
        constraint fk_crates_categories_category_id
            references categories
            on delete cascade,
    constraint crates_categories_pkey
        primary key (crate_id, category_id)
);

create index index_crates_categories_crate_id
    on crates_categories (crate_id);

create index index_crates_categories_category_id
    on crates_categories (category_id);

--create trigger trigger_update_categories_crates_cnt
--    before insert or delete
--    on crates_categories
--    for each row
--execute procedure update_categories_crates_cnt();
--
--create trigger touch_crate_on_modify_categories
--    after insert or delete
--    on crates_categories
--    for each row
--execute procedure touch_crate();

create table badges
(
    crate_id   integer not null
        constraint fk_badges_crate_id
            references crates
            on delete cascade,
    badge_type varchar not null,
    attributes jsonb   not null,
    constraint badges_pkey
        primary key (crate_id, badge_type)
);

create table reserved_crate_names
(
    name text not null
        constraint reserved_crate_names_pkey
            primary key
);

--create trigger trigger_ensure_reserved_name_not_in_use
--    before insert or update
--    on reserved_crate_names
--    for each row
--execute procedure ensure_reserved_name_not_in_use();

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

create unique index api_tokens_token_idx
    on api_tokens (token);

CREATE FUNCTION random_string(int4) RETURNS text AS $$
  SELECT (array_to_string(array(
    SELECT substr(
      'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789',
      floor(random() * 62)::int4 + 1,
      1
    ) FROM generate_series(1, $1)
  ), ''))
$$ LANGUAGE SQL;

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
    token              text    default random_string(26) not null,
    token_generated_at timestamp
);

--create trigger trigger_emails_set_token_generated_at
--    before insert or update
--        of token
--    on emails
--    for each row
--execute procedure emails_set_token_generated_at();
--
--create trigger trigger_emails_reconfirm
--    before update
--    on emails
--    for each row
--execute procedure reconfirm_email_on_email_change();

create table crate_owner_invitations
(
    invited_user_id    integer                             not null
        constraint crate_owner_invitations_invited_user_id_fkey
            references users
            on delete cascade,
    invited_by_user_id integer                             not null
        constraint crate_owner_invitations_invited_by_user_id_fkey
            references users
            on delete cascade,
    crate_id           integer                             not null
        constraint crate_owner_invitations_crate_id_fkey
            references crates
            on delete cascade,
    created_at         timestamp default now()             not null,
    token              text      default random_string(26) not null,
    token_generated_at timestamp,
    constraint crate_owner_invitations_pkey
        primary key (invited_user_id, crate_id)
);

--create trigger trigger_crate_owner_invitations_set_token_generated_at
--    before insert or update
--        of token
--    on crate_owner_invitations
--    for each row
--execute procedure crate_owner_invitations_set_token_generated_at();

create table readme_renderings
(
    version_id  integer                             not null
        constraint readme_renderings_pkey
            primary key
        constraint readme_renderings_version_id_fkey
            references versions
            on delete cascade,
    rendered_at timestamp default CURRENT_TIMESTAMP not null
);

create table background_jobs
(
    id         bigserial                                                            not null
        constraint background_jobs_pkey
            primary key,
    job_type   text                                                                 not null,
    data       jsonb                                                                not null,
    retries    integer   default 0                                                  not null,
    last_retry timestamp default '1970-01-01 00:00:00'::timestamp without time zone not null,
    created_at timestamp default CURRENT_TIMESTAMP                                  not null
);

create table version_owner_actions
(
    id           serial                  not null
        constraint version_owner_actions_pkey
            primary key,
    version_id   integer                 not null
        constraint version_owner_actions_version_id_fkey
            references versions
            on delete cascade,
    user_id      integer                 not null
        constraint version_owner_actions_owner_id_fkey
            references users,
    api_token_id integer
        constraint version_owner_actions_owner_token_id_fkey
            references api_tokens,
    action       integer                 not null,
    time         timestamp default now() not null
);

create index index_version_owner_actions_by_version_id
    on version_owner_actions (version_id);

create table versions_published_by
(
    version_id integer not null
        constraint versions_published_by_pkey
            primary key
        constraint versions_published_by_version_id_fkey
            references versions
            on delete cascade,
    email      varchar not null
);

create table publish_limit_buckets
(
    user_id     integer                             not null
        constraint publish_limit_buckets_pkey
            primary key
        constraint publish_limit_buckets_user_id_fkey
            references users,
    tokens      integer                             not null,
    last_refill timestamp default CURRENT_TIMESTAMP not null
);

create table publish_rate_overrides
(
    user_id integer not null
        constraint publish_rate_overrides_pkey
            primary key
        constraint publish_rate_overrides_user_id_fkey
            references users,
    burst   integer not null
);


