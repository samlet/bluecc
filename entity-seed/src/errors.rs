use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    // #[error("dotenv error")]
    // DotEnv(#[from] dotenv::Error),
    // #[error("env error")]
    // Env(#[from] std::env::VarError),
    #[error("parse error")]
    Parse(std::num::ParseIntError),
    #[error("xml parse fail")]
    ParseXml(roxmltree::Error),
    #[error("json parse fail")]
    ParseJson(#[from] serde_json::Error),
    #[error("request fail")]
    RequestErr(#[from] reqwest::Error),
    #[error("tera template fail")]
    TeraTemplateErr(#[from] tera::Error),
    #[error("database error")]
    DatabaseErr(#[from] quaint::error::Error),
    #[error("diesel error")]
    DieselErr(#[from] diesel::result::Error),
    #[error("config toml error")]
    ConfigTomlErr(#[from] toml::de::Error),
    #[error("glob pattern error")]
    PatternErr(#[from] glob::PatternError),
    #[error("glob error")]
    GlobErr(#[from] glob::GlobError),
    #[error("config error")]
    ConfigErr,
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("Error reading script file: {file_name:?}; {info:?}")]
    ScriptError {
        file_name: String,
        info: String,
    },
    #[error("Error finding: {item_name:?}; {info:?}")]
    NotFound {
        item_name: String,
        info: String,
    },
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("dotenv error")]
    DotEnv(#[from] dotenv::Error),
    #[error("env error")]
    Env(#[from] std::env::VarError),
}

impl From<roxmltree::Error> for GenericError {
    fn from(err: roxmltree::Error) -> GenericError {
        GenericError::ParseXml(err)
    }
}

#[derive(Debug)]
pub enum ErrorType {
    NotFound,
    Internal,
    BadRequest,
}

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError { message: message.to_string(), err_type }
    }

    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => {
                    match db_err {
                        diesel::result::DatabaseErrorKind::UniqueViolation => ErrorType::BadRequest,
                        _ => ErrorType::Internal,
                    }
                }
                diesel::result::Error::NotFound => ErrorType::NotFound,
                // If needed we can handle other cases
                _ => {
                    ErrorType::Internal
                }
            },
        )
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
