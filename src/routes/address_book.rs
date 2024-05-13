use axum::extract::{rejection::JsonRejection, Json, Path, Query, State};

use crate::repositories::address_book_repo::AddressBookRepository;
use crate::services::address_book_service::AddressBookService;
use crate::types::address_book::NewAddressBook;
use crate::types::{ApiError, ApiResponse, AppState, Pagination};

use handle_errors::Error;

pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<Pagination>,
) -> Result<ApiResponse, ApiError> {
    let limit = params.limit.unwrap_or(1);
    let offset = params.offset.unwrap_or(0);
    let repo = AddressBookRepository::new(state.pool);

    match AddressBookService::get_all_address_books(repo, Some(limit), offset).await {
        Ok(address_books) => Ok(ApiResponse::JsonDataAddressBookCollection(address_books)),
        Err(e) => Err(map_error(e)),
    }
}

pub async fn create_address_book(
    State(state): State<AppState>,
    payload: Result<Json<NewAddressBook>, JsonRejection>,
) -> Result<ApiResponse, ApiError> {
    match payload {
        Ok(payload) => {
            let address_book = payload.0;
            let repo = AddressBookRepository::new(state.pool);

            match AddressBookService::add_address_book(repo, address_book).await {
                Ok(address_book) => Ok(ApiResponse::JsonDataAddressBook(address_book)),
                Err(e) => Err(map_error(e)),
            }
        }
        Err(e) => Err(map_error(Error::JsonDeserilizationError(e))),
    }
}

pub async fn show(
    Path(address_book_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<ApiResponse, ApiError> {
    let repo = AddressBookRepository::new(state.pool);

    match AddressBookService::get_address_book_by_id(repo, address_book_id).await {
        Ok(address_book) => Ok(ApiResponse::JsonDataAddressBook(address_book)),
        Err(e) => Err(map_error(e)),
    }
}

pub async fn delete_address_book(
    Path(address_book_id): Path<i32>,
    State(state): State<AppState>,
) -> Result<ApiResponse, ApiError> {
    let repo = AddressBookRepository::new(state.pool);

    match AddressBookService::delete_address_book(repo, address_book_id).await {
        Ok(_) => Ok(ApiResponse::NoContent),
        Err(e) => Err(map_error(e)),
    }
}

pub async fn update(
    Path(id): Path<i32>,
    Json(address_book): Json<NewAddressBook>,
    State(state): State<AppState>,
) -> Result<ApiResponse, ApiError> {
    let repo = AddressBookRepository::new(state.pool);

    match AddressBookService::update_address_book(repo, id, address_book).await {
        Ok(address_book) => Ok(ApiResponse::JsonDataAddressBook(address_book)),
        Err(e) => Err(map_error(e)),
    }
}

fn map_error(error: Error) -> ApiError {
    match error {
        Error::DatabaseQueryError(_) => ApiError::DataBaseError,
        Error::AddressBookNotFound => ApiError::AddressBookNotFound,
        Error::JsonDeserilizationError(_) => ApiError::JsonDeserilize,
    }
}
