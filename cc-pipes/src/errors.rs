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
    #[error("json parse fail")]
    ParseJson(#[from] serde_json::Error),
    #[error("config toml error")]
    ConfigTomlErr(#[from] toml::de::Error),
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
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}
