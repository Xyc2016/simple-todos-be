use std::env;

use dotenvy::dotenv;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn build_from_env() -> Self {
        dotenv().ok();
        Config {
            database_url: env::var("DATABASE_URL").expect("PG_URL must be set"),
        }
    }
}

pub fn get_config() -> Config {
    Config::build_from_env()
}
