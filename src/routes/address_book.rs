use axum::extract::{Json, Path, Query, State};

use crate::repositories::address_book_repo::AddressBookRepository;
use crate::services::address_book_service::AddressBookService;
use crate::types::address_book::NewAddressBook;
use crate::types::{
    address_book::AddressBook, 
    ApiError, 
    ApiResponse, 
    AppState, 
    LoadingStrategy, 
    QueryParams,
};

use handle_errors::Error;
use serde_json::json;

pub async fn index(
    State(state): State<AppState>,
    Query(query_params): Query<QueryParams>,
) -> Result<ApiResponse<Vec<AddressBook>>, ApiError> {
    let repo = AddressBookRepository::new(state.pool);

    let address_book_service = AddressBookService::new(repo);
    let limit = query_params.limit;
    let offset = query_params.offset;

    let load_strategy = match query_params.loading_strategy {
        Some(LoadingStrategy::Lazy) => "lazy",
        Some(LoadingStrategy::Eager) => "eager",
        None => "eager",
    };

    match address_book_service
        .get_all_address_books(Some(limit), offset, load_strategy)
        .await
    {
        Ok(address_books) => Ok(ApiResponse::JsonData(address_books)),
        Err(_e) => Err(ApiError::InternalServerError(
            json!({"error": "something went wrong"}).to_string(),
        )),
    }
}

pub async fn create(
    Json(address_book): Json<NewAddressBook>,
    State(state): State<AppState>,
) -> Result<ApiResponse<AddressBook>, ApiError> {
    let repo = AddressBookRepository::new(state.pool);
    let address_book_service = AddressBookService::new(repo);
    match address_book_service.add_address_book(address_book).await {
        Ok(address_book) => Ok(ApiResponse::JsonData(address_book)),
        Err(_e) => Err(ApiError::InternalServerError(
            json!({"error": "something went wrong"}).to_string(),
        )),
    }
}

pub async fn show(
    Path(address_book_id): Path<i32>,
    State(state): State<AppState>,
    Query(query_params): Query<QueryParams>,
) -> Result<ApiResponse<AddressBook>, ApiError> {
    let repo = AddressBookRepository::new(state.pool);
    let address_book_service = AddressBookService::new(repo);

    let load_strategy = match query_params.loading_strategy {
        Some(LoadingStrategy::Lazy) => "lazy",
        Some(LoadingStrategy::Eager) => "eager",
        None => "eager",
    };

    match address_book_service
        .get_address_book_by_id(address_book_id, load_strategy)
        .await
    {
        Ok(address_book) => Ok(ApiResponse::JsonData(address_book)),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn delete(
    Path(address_book_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<ApiResponse<()>, ApiError> {
    let repo = AddressBookRepository::new(state.pool);
    let address_book_service = AddressBookService::new(repo);
    match address_book_service
        .delete_address_book(address_book_id)
        .await
    {
        Ok(_) => Ok(ApiResponse::NoContent),
        Err(e) => Err(handle_error(e)),
    }
}

fn handle_error(error: Error) -> ApiError {
    match error {
        Error::DatabaseQueryError(_) => {
            ApiError::InternalServerError(json!({"error": "something went wrong"}).to_string())
        }
        Error::InvalidLoadStrategy => {
            ApiError::BadRequest(json!({"error": "invalid load strategy"}).to_string())
        }
        Error::AddressBookNotFound => {
            ApiError::NotFound(json!({"error": "address book not found"}).to_string())
        }
    }
}
