// auth.rs
use axum::{
    extract::{Form, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login(
    pool: Arc<PgPool>,
    Form(payload): Form<LoginRequest>,
) -> impl IntoResponse {
    // Authenticate and generate JWT (or session)
    // This is just a placeholder for actual auth logic
    let user = sqlx::query!("SELECT * FROM players WHERE username = $1", payload.username)
        .fetch_optional(&*pool)
        .await
        .unwrap();

    match user {
        Some(_user) => {
            // Generate a token (mock token here)
            let token = "mock-token";
            Json(AuthResponse {
                token: token.to_string(),
            })
        }
        None => StatusCode::UNAUTHORIZED.into_response(),
    }
}
