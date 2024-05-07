pub mod address_book;
pub mod contact;

use axum::{
    response::{IntoResponse, Response},
    Json,
};


use axum::http::StatusCode;
use serde_json::json;

use self::address_book::AddressBook;

#[derive(serde::Deserialize)]
pub enum LoadingStrategy {
    #[serde(rename = "lazy")]
    Lazy,
    #[serde(rename = "eager")]
    Eager,
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub enum ApiResponse {
    JsonDataAddressBook(AddressBook),
    JsonDataAddressBookCollection(Vec<AddressBook>),
    NoContent,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::JsonDataAddressBook(data) => {
                (StatusCode::OK, Json(data)).into_response()
            }
            ApiResponse::JsonDataAddressBookCollection(data) => {
                (StatusCode::OK, Json(data)).into_response()
            }
            ApiResponse::NoContent => (StatusCode::NO_CONTENT).into_response(),
        }
    }
}

pub enum ApiError {
    DataBaseError,
    JsonDeserilize,
    AddressBookNotFound,
    
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg) = match self {
            ApiError::DataBaseError => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong"),
            ApiError::JsonDeserilize => (StatusCode::BAD_REQUEST, "Json deserialization error"),
            ApiError::AddressBookNotFound => (StatusCode::NOT_FOUND, "addressbook not found"),
        };

        let body = Json(json!({
            "error": error_msg

        }));
        (status, body).into_response()

    }
}
