use std::env;
use dotenvy::dotenv;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Environment {
    Development,
    Production,
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(input: &str) -> Result<Environment, Self::Err> {
        match input.to_uppercase().as_str() {
            "DEVELOPMENT" => Ok(Environment::Development),
            "PRODUCTION" => Ok(Environment::Production),
            _ => Ok(Environment::Development), // Default
        }
    }
}

pub fn get_env() -> Environment {
    dotenv().ok();
    let env_str = env::var("ENVIRONMENT").unwrap_or_else(|_| "DEVELOPMENT".to_string());
    Environment::from_str(&env_str).unwrap()
}
