pub mod address_book;
pub mod contact;
pub mod pagination;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use axum::http::StatusCode;

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
    pub loading_strategy: Option<LoadingStrategy>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}

pub enum ApiResponse<T: Serialize> {
    JsonData(T),
    NoContent,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
            ApiResponse::NoContent => (StatusCode::NO_CONTENT).into_response(),
        }
    }
}

pub enum ApiError {
    InternalServerError(String),
    BadRequest(String),
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::InternalServerError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
        }
    }
}
