mod error;
mod load;

use error::EnvError;
use load::load_env_var;

pub type SuggestionNumber = usize;

// I could make it just a return a tuple of values like this (host, port, suggestion_number)
// But if we accidentally mixed the values, it could lead to hard-to-find bugs.

pub struct EnvVars {
    pub host: String,
    pub port: String,
    pub suggestion_number: SuggestionNumber,
}

impl EnvVars {
    pub fn new() -> Result<EnvVars, EnvError> {
        load_locally();

        let host: String = match load_env_var("HOST") {
            Ok(value) => value,
            Err(_) => "0.0.0.0".to_string(),
        };

        let port: String = load_env_var("PORT")?;
        let suggestion_number: String = load_env_var("SUGGESTION_NUMBER")?;

        let suggestion_number = match suggestion_number.parse::<SuggestionNumber>() {
            Ok(value) => value,
            Err(_) => {
                return Err(EnvError::InvalidValueNumber(String::from(
                    "SUGGESTION_NUMBER",
                )))
            }
        };

        Ok(EnvVars {
            host,
            port,
            suggestion_number,
        })
    }
}

/// If the feature "with-dotenv" is enabled. Load local .env environment variables."
#[cfg(feature = "dotenv")]
fn load_locally() {
    match dotenv::dotenv() {
        Err(err) => println!("Failed to load .env file. Error:{err}.\n\n Continuing..."),
        Ok(_) => {}
    }
}

/// If the feature "with-dotenv" is disabled. This function does nothing. Used in the docker instance.
#[cfg(all(not(feature = "dotenv"), not(test)))]
fn load_locally() {
    // Doesn't do anything when feature is not enabled.
}

/// Set environment variables that only exist in testing
#[cfg(test)]
fn load_locally() {
    std::env::set_var("HOST", "0.0.0.0");
    std::env::set_var("PORT", "3030");
    std::env::set_var("SUGGESTION_NUMBER", "10");
}
