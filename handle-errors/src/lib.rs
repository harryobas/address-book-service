use sqlx::Error as SqlxError;
use thiserror::Error;
use axum::extract::rejection::JsonRejection as JsonRejection;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Query could not be executed: {0}")]
    DatabaseQueryError(#[from] SqlxError),
    #[error("AddressBook not found")]
    AddressBookNotFound,
    //#[error("Missing parameters")]
    //MissingParameters,
    #[error("Ivalid json string")]
    JsonDeserilizationError(#[from] JsonRejection)
}
