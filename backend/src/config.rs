use std::env;
use dotenvy::dotenv;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
