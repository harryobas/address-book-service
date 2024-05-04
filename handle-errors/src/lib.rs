use sqlx::Error as SqlxError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot parse parameter: {0}")]
    ParseError(#[from] ParseIntError),
    #[error("Query could not be executed: {0}")]
    DatabaseQueryError(#[from] SqlxError),
    #[error("AddressBook not found")]
    AddressBookNotFound,
    #[error("Missing parameters")]
    MissingParameters,
    #[error("Ivalid loading strategy")]
    InvalidLoadStrategy,
}
