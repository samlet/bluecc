use chrono::NaiveDateTime;
use diesel::associations::Identifiable;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::Bool;
use url::Url;

use crate::schema::*;

// #[derive(Debug, Queryable, Identifiable, Associations, Clone, Copy)]
// #[belongs_to(Crate)]
// #[primary_key(crate_id)]
// #[table_name = "recent_crate_downloads"]
// pub struct RecentCrateDownloads {
//     pub crate_id: i32,
//     pub downloads: i32,
// }

#[derive(Debug, Clone, Queryable, Identifiable, Associations, AsChangeset, QueryableByName)]
#[table_name = "crates"]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub downloads: i32,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub max_upload_size: Option<i32>,
}

/// We literally never want to select `textsearchable_index_col`
/// so we provide this type and constant to pass to `.select`
type AllColumns = (
    crates::id,
    crates::name,
    crates::updated_at,
    crates::created_at,
    crates::downloads,
    crates::description,
    crates::homepage,
    crates::documentation,
    crates::repository,
    crates::max_upload_size,
);

pub const ALL_COLUMNS: AllColumns = (
    crates::id,
    crates::name,
    crates::updated_at,
    crates::created_at,
    crates::downloads,
    crates::description,
    crates::homepage,
    crates::documentation,
    crates::repository,
    crates::max_upload_size,
);

pub const MAX_NAME_LENGTH: usize = 64;
