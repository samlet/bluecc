use thiserror::Error;

pub mod permissions;
mod data_format;
mod adapters;

/// General acl error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Casbin Role Manager Error: `{0:?}`")]
    RbacError(#[from] casbin::Error),
    #[error("Service error")]
    ServiceErr(#[from] deles::ServiceError),
}

pub type Result<T> = std::result::Result<T, Error>;
pub use data_format::{role_lines, perm_lines};