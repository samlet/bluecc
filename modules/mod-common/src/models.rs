#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub use self::user::{NewUser, User};

pub mod user;
mod post;
mod follow;
mod email;
mod book;
