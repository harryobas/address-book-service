mod repositories;
mod routes;
mod services;
mod types;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use routes::address_book::*;
use sqlx::PgPool;
use types::AppState;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Faild to run migrations");

    let state = AppState { pool };

    let router = Router::new()
        .route("/api/addressbooks", get(index))
        .route("/api/addressbooks", post(create_address_book))
        //.route("/api/addressbooks/:id", put(update))
        .route("/api/addressbooks/:id", delete(delete_address_book))
        .route("/api/addressbooks/:id", get(show))
        .with_state(state);

    Ok(router.into())
}
