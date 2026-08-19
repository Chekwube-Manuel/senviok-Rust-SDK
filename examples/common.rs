#![allow(dead_code)]

use senviok::Senviok;
use std::env;

fn load_env() {
    match dotenvy::dotenv() {
        Ok(_) | Err(dotenvy::Error::Io(_)) => {}
        Err(error) => panic!("failed to parse .env: {error}"),
    }
}

pub fn client() -> Senviok {
    load_env();
    let api_key = env::var("SENVIOK_API_KEY").expect("SENVIOK_API_KEY must be set");
    Senviok::new(api_key)
}

pub fn require_env(key: &str) -> String {
    load_env();
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}

pub fn optional_env(key: &str) -> Option<String> {
    load_env();
    env::var(key).ok().filter(|value| !value.is_empty())
}
