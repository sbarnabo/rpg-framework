use axum::{Router, routing::get, Extension};
use sqlx::PgPool;

use crate::routes::health_check;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(Extension(pool))
}
