/// Log messages that only exist in development mode.
#[cfg(all(debug_assertions, not(test)))]
pub fn log_debug(message: &str) {
    println!("{}", message);
}

#[cfg(not(debug_assertions))]
pub fn log_debug(_message: &str) {
    // Does nothing during production
}

#[cfg(test)]
pub fn log_debug(message: &str) {
    println!("{}", message);
    // Does nothing during tests
}
