use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

// A single Entry of Output
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    name: String,
    times: u64,
}

impl Entry {
    pub fn new(name: &str, times: u64) -> Self {
        Entry {
            name: name.into(),
            times,
        }
    }

    pub fn get_name<'a>(&'a self) -> &'a str{
        &self.name
    }

    pub fn get_times<'a>(&'a self) -> &'a u64{
        &self.times
    }

    pub fn get_times_mut<'a>(&'a mut self) -> &'a mut u64 {
        &mut self.times
    }
}

// When displaying this error for the user, we need to be able to turn it into a String.
impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deserialized = match serde_json::to_string(&self) {
            Ok(value) => value,
            Err(err) => panic!(
                "Error Deserializing Entry. Name: {}, Times: {}, Err: {}.",
                self.name, self.times, err
            ),
        };

        write!(f, "{}", deserialized)
    }
}

// Ord, PartialOrd, Eq and PartialEq must be implemented for sorting a Vec of Entry.
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare times in reverse for ordering
        match other.times.cmp(&self.times) {
            Ordering::Equal => {
                let first = &self.name;
                let second = &other.name;

                // If both are equal, compare the names
                // It's not in reverse
                first.cmp(second)
            }
            other => other,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.times == other.times
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    /// Assert that ordering is correct
    #[test]
    fn entry_ordering() {
        let mut entries: Vec<Entry> = Vec::from([
            Entry::new("Lucas", 3),
            Entry::new("Ann", 10),
            Entry::new("Annette", 500),
            Entry::new("Isabella", 31),
            Entry::new("Minamoto", 2),
            Entry::new("Robert", 5),
            Entry::new("Otoharada", 10),
            Entry::new("Kaguya", 3),
        ]);

        entries.sort();

        let sorted = Vec::from([
            Entry::new("Annette", 500),
            Entry::new("Isabella", 31),
            Entry::new("Ann", 10),
            Entry::new("Otoharada", 10),
            Entry::new("Robert", 5),
            Entry::new("Kaguya", 3),
            Entry::new("Lucas", 3),
            Entry::new("Minamoto", 2),
        ]);

        for i in 0..entries.len() {
            let first = &entries[i];
            let second = &sorted[i];

            let comparison: bool =
                first.name.eq_ignore_ascii_case(&second.name) && first.times == second.times;

            assert!(
                comparison,
                "Comparison failed. First {first}, Second {second} ."
            );
        }
    }
}
