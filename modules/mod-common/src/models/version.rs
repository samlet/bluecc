use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use diesel::prelude::*;

// use crate::models::{Crate, Dependency, User};
use crate::models::{Crate};
use crate::schema::*;

// Queryable has a custom implementation below
#[derive(Clone, Identifiable, Associations, Debug, Queryable, Deserialize, Serialize)]
#[belongs_to(Crate)]
pub struct Version {
    pub id: i32,
    pub crate_id: i32,
    pub num: semver::Version,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub downloads: i32,
    pub features: serde_json::Value,
    pub yanked: bool,
    pub license: Option<String>,
    pub crate_size: Option<i32>,
    pub published_by: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "versions"]
pub struct NewVersion {
    crate_id: i32,
    num: String,
    features: serde_json::Value,
    license: Option<String>,
    crate_size: Option<i32>,
    published_by: i32,
}

/// The highest version (semver order) and the most recently updated version.
/// Typically used for a single crate.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopVersions {
    /// The "highest" version in terms of semver
    pub highest: Option<semver::Version>,
    /// The "highest" non-prerelease version
    pub highest_stable: Option<semver::Version>,
    /// The "newest" version in terms of publishing date
    pub newest: Option<semver::Version>,
}

