use std::fmt::Display;

// While this Enum only has a single value,
// Keeping it makes it easier to extend the code.

pub enum GetPrefixError {
    NotFound(String),
}

impl Display for GetPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(prefix) => write!(f, "Prefix {} not found", prefix),
        }
    }
}
