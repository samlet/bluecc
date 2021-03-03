use chrono::NaiveDateTime;
use diesel::prelude::*;
use std::borrow::Cow;

use crate::schema::{crate_owners, emails, users};

/// The model representing a row in the `users` database table.
#[derive(Clone, Debug, PartialEq, Eq, Queryable, Identifiable, AsChangeset, Associations)]
pub struct User {
    pub id: i32,
    pub gh_access_token: String,
    pub gh_login: String,
    pub name: Option<String>,
    pub gh_avatar: Option<String>,
    pub gh_id: i32,
    pub account_lock_reason: Option<String>,
    pub account_lock_until: Option<NaiveDateTime>,
}

/// Represents a new user record insertable to the `users` table
#[derive(Insertable, Debug, Default)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub gh_id: i32,
    pub gh_login: &'a str,
    pub name: Option<&'a str>,
    pub gh_avatar: Option<&'a str>,
    pub gh_access_token: Cow<'a, str>,
}

impl<'a> NewUser<'a> {
    pub fn new(
        gh_id: i32,
        gh_login: &'a str,
        name: Option<&'a str>,
        gh_avatar: Option<&'a str>,
        gh_access_token: &'a str,
    ) -> Self {
        NewUser {
            gh_id,
            gh_login,
            name,
            gh_avatar,
            gh_access_token: Cow::Borrowed(gh_access_token),
        }
    }
}
