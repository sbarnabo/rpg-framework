use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;

pub async fn health_check(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
