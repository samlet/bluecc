use thiserror::Error;

mod permissions;
mod data_format;

/// General acl error
#[derive(Error, Debug)]
pub enum Error {
    #[error("Casbin Role Manager Error: `{0:?}`")]
    RbacError(#[from] casbin::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
