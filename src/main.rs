mod repositories;
mod routes;
mod services;
mod types;

use axum::{routing::get, Router};
use sqlx::PgPool;
use types::AppState;
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Faild to run migrations");

    let state = AppState { pool };

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
