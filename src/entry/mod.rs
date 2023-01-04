use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use serde::{Deserialize, Serialize};

/// The main data type stored by the Nodes.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    name: String,
    times: u64,
}

impl Entry {
    /// Constructs an Entry with given name and times.
    pub fn new(name: String, times: u64) -> Self {
        Entry { name, times }
    }

    /// Return a reference to name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Return a reference to times.
    pub fn get_times(&self) -> &u64 {
        &self.times
    }

    /// Return a mutable reference to times.
    pub fn get_times_mut(&mut self) -> &mut u64 {
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

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // compiler will implement branchless programming here
        let greater = self.gt(other);
        let equal = self.eq(other);
        let less = self.lt(other);

        match (greater, equal, less) {
            (true, _, _) => Ordering::Greater,
            (_, true, _) => Ordering::Equal,
            (_, _, true) => Ordering::Less,
            (_, _, _) => unreachable!(),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn ge(&self, other: &Self) -> bool {
        self.times >= other.times
            || (self.times == other.times)
                && (self.name.to_ascii_lowercase() <= other.name.to_ascii_lowercase())
    }

    fn le(&self, other: &Self) -> bool {
        self.times <= other.times
            || (self.times == other.times)
                && (self.name.to_ascii_lowercase() >= other.name.to_ascii_lowercase())
    }

    fn gt(&self, other: &Self) -> bool {
        self.times > other.times
            || (self.times == other.times)
                && (self.name.to_ascii_lowercase() < other.name.to_ascii_lowercase())
    }

    fn lt(&self, other: &Self) -> bool {
        self.times < other.times
            || (self.times == other.times)
                && (self.name.to_ascii_lowercase() > other.name.to_ascii_lowercase())
    }
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.times == other.times && self.name.eq_ignore_ascii_case(&other.name)
    }
}

impl From<(&str, u64)> for Entry {
    fn from(value: (&str, u64)) -> Self {
        let (name, times) = value;

        Entry {
            name: name.to_string(),
            times,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;
    use std::cmp::Ordering;

    fn cmp_expect(first: &Entry, second: &Entry, expected: Ordering) {
        let comparison = first.cmp(second);
        if !comparison.eq(&expected) {
            panic!(
                "{first} == {second} Expected {:#?}, got {:#?} .",
                expected, comparison
            );
        }
    }

    #[test]
    fn cmp_tests() {
        let first = Entry::new("Ann".into(), 50);
        let second = Entry::new("Macbeth".into(), 50);
        let third = Entry::new("Joseph".into(), 100);
        let fourth = Entry::new("ann".into(), 50);

        cmp_expect(&first, &second, Ordering::Greater);
        cmp_expect(&first, &third, Ordering::Less);
        cmp_expect(&first, &fourth, Ordering::Equal);
        cmp_expect(&second, &third, Ordering::Less);
        cmp_expect(&second, &fourth, Ordering::Less);
        cmp_expect(&third, &fourth, Ordering::Greater);
    }

    /// Assert that ordering is correct
    #[test]
    fn entry_ordering() {
        let mut entries: Vec<Entry> = Vec::from([
            Entry::new("Lucas".into(), 3),
            Entry::new("Ann".into(), 10),
            Entry::new("Annette".into(), 500),
            Entry::new("Isabella".into(), 31),
            Entry::new("Minamoto".into(), 2),
            Entry::new("Robert".into(), 5),
            Entry::new("Otoharada".into(), 10),
            Entry::new("Kaguya".into(), 3),
        ]);

        entries.sort_by(|a, b| b.cmp(a));

        let sorted = Vec::from([
            Entry::new("Annette".into(), 500),
            Entry::new("Isabella".into(), 31),
            Entry::new("Ann".into(), 10),
            Entry::new("Otoharada".into(), 10),
            Entry::new("Robert".into(), 5),
            Entry::new("Kaguya".into(), 3),
            Entry::new("Lucas".into(), 3),
            Entry::new("Minamoto".into(), 2),
        ]);

        for i in 0..entries.len() {
            let first = &entries[i];
            let second = &sorted[i];

            let comparison: bool =
                first.name.eq_ignore_ascii_case(&second.name) && first.times == second.times;

            if !comparison {
                let mut message: String = "Acquired entries (acquired/expected):\n".into();

                for index in 0..entries.len() {
                    let acquired = &entries[index];
                    let expected = &sorted[index];

                    message = format!("{message}\n{acquired} == {expected}");
                }
                message = format!("{message}\n\nComparison Failed...\n\n");

                panic!("{message}");
            }
        }
    }
}
