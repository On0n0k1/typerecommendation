use std::env;

use crate::{env::EnvError, log::log_debug};

pub fn load_env_var(key: &str) -> Result<String, EnvError> {
    match env::var(key) {
        Ok(value) => {
            log_debug(&format!("Environment Variable {key}: {value}"));
            Ok(value)
        }
        Err(err) => {
            let key = String::from(key);

            match err {
                env::VarError::NotPresent => Err(EnvError::NotFound(key)),
                env::VarError::NotUnicode(_) => Err(EnvError::InvalidValueUnicode(key)),
            }
        }
    }
}
