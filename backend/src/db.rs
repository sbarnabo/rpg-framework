use sqlx::{Pool, Postgres, postgres::PgPoolOptions, migrate::Migrator};
use std::{env, sync::Arc};
use dotenvy::dotenv;

pub type DbPool = Arc<Pool<Postgres>>;

// Migrations directory
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Initialize the database connection pool and run migrations.
pub async fn init_db() -> DbPool {
    dotenv().ok(); // Load from .env in dev environments

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to PostgreSQL");

    MIGRATOR.run(&pool)
        .await
        .expect("Database migrations failed");
    
    println!("✅ Database connected and migrations applied");
    Arc::new(pool)
}
/// Check database health by executing a lightweight query.
pub async fn check_db_health(pool: &DbPool) -> bool {
    sqlx::query("SELECT 1")
        .execute(pool.as_ref())
        .await
        .is_ok()
}
/// Optional: Seed the database with initial data for development/testing.
pub async fn seed_data(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Example: insert a default region or player class
    sqlx::query!(
        r#"
        INSERT INTO character_classes (name, description)
        VALUES ($1, $2)
        ON CONFLICT (name) DO NOTHING
        "#,
        "Adventurer",
        "A brave soul starting their journey."
    )
    .execute(pool.as_ref())
    .await?;

    println!("🌱 Seed data inserted");
    Ok(())
}
