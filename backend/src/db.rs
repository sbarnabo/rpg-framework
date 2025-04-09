use sqlx::{Pool, Postgres, postgres::PgPoolOptions, migrate::Migrator};
use std::env;
use std::sync::Arc;
use dotenvy::dotenv;

pub type DbPool = Arc<Pool<Postgres>>;

// Embed the migrations from the `migrations/` directory at compile time
pub static MIGRATOR: Migrator = sqlx::migrate!();

/// Initialize the database connection pool and run migrations.
pub async fn init_db() -> DbPool {
    dotenv().ok(); // Load from .env in dev environments

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the Postgres database");

    MIGRATOR.run(&pool)
        .await
        .expect("Database migrations failed");

    Arc::new(pool)
}
