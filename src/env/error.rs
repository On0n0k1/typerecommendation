use std::fmt::Display;

pub enum EnvError {
    NotFound(String),
    InvalidValueUnicode(String),
    InvalidValueNumber(String),
}

impl Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EnvError::NotFound(key) => write!(f, "Key {} Not Found", key),
            EnvError::InvalidValueUnicode(key) => {
                write!(f, "Value for key {} has an invalid Unicode format", key)
            }
            EnvError::InvalidValueNumber(key) => write!(f, "Invalid number format for key {}", key),
        }
    }
}
